/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Clone + Ord> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut current = self.start;
        for _ in 0..index {
            current = Some(current.and_then(|n| unsafe { (*n.as_ptr()).next })?);
        }
        current.map(|n| unsafe { &(n.as_ref().val) })
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut result = LinkedList::new();
        let (mut current_a, mut current_b) = (list_a.start, list_b.start);
    
        while let (Some(node_a), Some(node_b)) = (current_a, current_b) {
            let node_a = unsafe { &*node_a.as_ptr() };
            let node_b = unsafe { &*node_b.as_ptr() };
    
            if node_a.val < node_b.val {
                result.add(node_a.val.clone());
                current_a = node_a.next;
            } else {
                result.add(node_b.val.clone());
                current_b = node_b.next;
            }
        }
    
        // Append the remaining nodes from the non-empty list.
        let mut current = current_a.or(current_b);
        while let Some(node) = current {
            let node = unsafe { &*node.as_ptr() };
            result.add(node.val.clone());
            current = node.next;
        }
    
        result
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;
        while let Some(node) = current {
            let node = unsafe { &*node.as_ptr() };
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", node.val)?;
            current = node.next;
            first = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        
        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i).unwrap());
        }
    }
    
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in vec_a {
            list_a.add(i);
        }
        for i in vec_b {
            list_b.add(i);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i).unwrap());
        }
    }
}