/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Ord + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        let idx = self.count;
        self.count += 1;
        self.sift_up(idx);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        (idx - 1) / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 2
    }

    fn smallest_child_idx(&self, idx: usize) -> Option<usize> {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        
        if left >= self.count {
            return None;
        }
        
        if right >= self.count {
            // If only the left child exists, return left
            return Some(left);
        }
        
        // Use the comparator to decide between left and right children
        if (self.comparator)(&self.items[left], &self.items[right]) {
            Some(left)
        } else {
            Some(right)
        }
    }
    

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent_idx = self.parent_idx(idx);
            if (self.comparator)(&self.items[parent_idx], &self.items[idx]) {
                break;
            }
            self.items.swap(idx, parent_idx);
            idx = parent_idx;
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        loop {
            let smallest_child_idx = self.smallest_child_idx(idx);
            match smallest_child_idx {
                Some(child_idx) if !(self.comparator)(&self.items[idx], &self.items[child_idx]) => {
                    self.items.swap(idx, child_idx);
                    idx = child_idx;
                }
                _ => break,
            }
        }
    }
    
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let item = self.items[0].clone();
            if self.count > 1 {
                self.items[0] = self.items.remove(self.count - 1);
                self.count -= 1;
                self.sift_down(0); // Start sifting from the root
            } else {
                self.items.clear();
                self.count = 0;
            }
            Some(item)
        }
    }
}


pub struct MinHeap;

impl MinHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}