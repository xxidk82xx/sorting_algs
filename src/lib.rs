
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

macro_rules! twoSorted {
    ($arr:expr, $i:expr, $j:expr) => {
        $arr[$i] <= $arr[$j]
    };
}

pub fn is_sorted<T:PartialOrd>(arr:&mut [T]) -> bool{
    for i in 1..arr.len(){
        if !twoSorted!(arr, i-1, i) {
            return false
        }
    }
    true
}

fn to_sorted<T:PartialOrd>(arr:&mut [T], to_move:usize) -> &[T] {
    for i in 0..to_move {
        if arr[to_move-i] < arr[to_move-i-1] {
            arr.swap(to_move-i, to_move-i-1);
        } else {
            return arr;
        }
    }
    arr
} 

pub trait SearchAlgs<T:PartialOrd> {
    fn bin_search(&self, target:T) -> Option<usize>;

    fn linear_search(&self, target:T) -> Option<usize>;
}

impl<T:PartialOrd> SearchAlgs<T> for [T] {
    ///uses a binary search to find a target designated by T in an array designated by arr   
    ///#Examples
    fn bin_search(&self, target:T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.len();
        
        while left != right - 1{
            let mid = (left + right)/2;
            if self[mid] == target {
                return Some(mid);
            } else if self[mid] < target {
                left = mid;
            } else if target < self[mid] {
                right = mid;
            }
        }
        None
    }
    ///this is slower because its O(N) while bin is O(log(N))
    fn linear_search(&self, target:T) -> Option<usize> {
        for i in 0..self.len() {
            if self[i] == target {
                return Some(i);
            }
        }
        None
    }
}   

pub trait SortingAlgs<T:PartialOrd> {
    /// fastest because it splits off logarithmiaclly however still NlogN
    fn quick_sort(&mut self) -> &mut Self;

    fn selection_sort(&mut self) -> &mut Self; 

    /// uses a bubble sort algorithm to sort a given array (slowest)
    /// # Examples
    /// ```rust
    /// use sorting_algs::*;
    /// let arr = &mut vec!(5, -10, 6, 2);
    ///   
    /// assert_eq!(arr.bubble_sort(), &mut *vec!(-10, 2, 5, 6))
    /// ``` 
    fn bubble_sort(&mut self) -> &mut Self;

    fn insertion_sort(&mut self) -> &mut Self;
}

impl<T:PartialOrd> SortingAlgs<T> for [T] {
    fn quick_sort(&mut self) -> &mut [T] {
        let length = self.len();
        if length <= 1 {
            return self;
        }
        let pivot = length-1;
        let mut greater = pivot;
        let mut swap = false;
        for i in 0..pivot {
            if !swap && self[i] > self[pivot] {
                greater = i;
                swap = true
            }
            if swap && self[i] < self[pivot] {
                self.swap(greater, i)
            }
        }
        self.swap(greater, pivot);
        let (left, right) = self.split_at_mut(greater);
        left.quick_sort();
        right.quick_sort();
        self
    }

    fn selection_sort(&mut self) -> &mut [T] {
        let length = self.len();
        for i in 0..length {
            let mut min = i;
            for j in i..self.len() {
                if self[min] > self[j] {
                    min = j;
                }
            }
            self.swap(i, min);
        }
        self
    }

    fn bubble_sort(&mut self) -> &mut [T] {
        let length = self.len();
        for i in 0..length {
            let mut sorted = true;
            for j in 1..length-i {
                if self[j-1] > self[j] {
                    self.swap(j-1, j);
                    sorted = false;
                }
            }
            if sorted {
                return self;
            }
        }
        self
    }   

    fn insertion_sort(&mut self) -> &mut [T] {
        let len = self.len();
        for _i in 0..len {
            let mut sorted = true;
            for j in 1..len {
                if self[j-1] > self[j] {
                    sorted = false;
                    to_sorted(self, j);
                }
            }
            if sorted {
                return self;
            }
        }
        self
    }

}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn _bin_search() {
        let mut arr:[i32;8] = rand::random();
        arr.sort();
        let target:i32 = arr[rand::random::<usize>()%8];
        let offset = arr.bin_search(target).unwrap();
        assert_eq!(arr[offset], target)
    }

    #[test]
    fn _bubble_sort() {
        let arr = &mut vec!(1, 6, 5, 2);
        assert_eq!(arr.bubble_sort(), &mut *vec!(1, 2, 5, 6))
    }

    #[test]
    fn _insertion_sort() {
        let arr = &mut vec!(1, 6, 5, 2);
        assert_eq!(arr.bubble_sort(), &mut *vec!(1, 2, 5, 6))
    }

    #[test]
    fn _selection_sort() {
        let arr = &mut vec!(1, 6, 5, 2);
        assert_eq!(arr.selection_sort(), &mut *vec!(1, 2, 5, 6))
    }

    #[test]
    fn _quick_sort() {
        let arr = &mut vec!(1, 6, 5, 2);
        assert_eq!(arr.quick_sort(), &mut *vec!(1, 2, 5, 6))
    }
}