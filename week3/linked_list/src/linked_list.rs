use std::fmt;
use std::option::Option;
use std::iter::Sum;
use std::ops::{Add, Mul};

pub trait ComputeNorm {
    fn compute_norm(&self) -> u64;
}


#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value, next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node: Box<Node<T>> = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node: Box<Node<T>> = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node<T>>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        LinkedList {
            head: {match &self.head {
                    Some(x) => Some(x.clone()),
                    None => None,
                    }
            },
            size: self.size,
        }

    }
}


impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.head == other.head 
    }
}

// Iterator trait
// This struct is necessary to keep the state of the iterator
pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}


// The iterator trait is implemented onto the LinkedListIter struct 
// as this is the struct keeping the iterator status.
// The resulting output is u32 (Item type)
impl<T: Copy> Iterator for LinkedListIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                self.current = &node.next;
                Some(node.value)
            },
            None => None,
        }
    }
}

// The IntoInterator trait is implemented onto the LinkedList
// to "convert" the type from LinkedList to LinkedListIter 
impl<'a, T: Clone + Copy> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter {current: &self.head}
    }
}


// the syntax Trait<T, Ass. Type> 
impl<T> ComputeNorm for LinkedList<T> 
    where
    T: Add<T, Output = u64> + Mul<u64, Output = u64> + Sum<u64> + Into<u64> + Copy,
{
    fn compute_norm(&self) -> u64 {
        let x: u64 = self.head.iter().map(|x| x.value * 2).sum::<T>().into();
        x
    }
}

