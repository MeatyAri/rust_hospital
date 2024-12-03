// use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::clone::Clone;
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::{data_structures::max_heap::MaxHeap, db::entities::UniqueAttribute};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriorityQueue<T: Ord + Clone + Debug> {
    heap: MaxHeap<Reverse<T>>,
}

impl<T: Ord + Clone + Debug> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: MaxHeap::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item.clone()));
    }

    pub fn insert(&mut self, item: T) {
        self.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(item)| item.clone())
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn get_by_uniq_attr(&mut self, uniq_attr: String) -> Option<&mut T>
    where 
        T: UniqueAttribute,
    {
        for i in 0..self.heap.size {
            if self.heap.data[i].as_mut().unwrap().0.uattr() == uniq_attr {
                return Some(&mut self.heap.data[i].as_mut().unwrap().0);
            }
        }
        None
    }

    pub fn remove_by_uniq_attr(&mut self, uniq_attr: String) -> bool
    where 
        T: UniqueAttribute,
    {
        for i in 0..self.heap.size {
            if self.heap.data[i].as_mut().unwrap().0.uattr() == uniq_attr {
                self.heap.data[i] = None;
                return true;
            }
        }
        false
    }
}

// fn main() {
//     let mut pq = PriorityQueue::new();

//     pq.push(5);
//     pq.push(1);
//     pq.push(10);
//     pq.push(3);

//     println!("Top element: {:?}", pq.peek());
    
//     while !pq.is_empty() {
//         println!("Popped: {:?}", pq.pop());
//     }
// }

