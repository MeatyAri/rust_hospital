// use std::collections::BinaryHeap;
use std::cmp::Reverse;

use crate::data_structures::max_heap::MaxHeap;

#[derive(Debug)]
pub struct PriorityQueue<T: Ord + Copy> {
    heap: MaxHeap<Reverse<T>>,
}

impl<T: Ord + Copy> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue {
            heap: MaxHeap::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(Reverse(item));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Reverse(item)| item)
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|Reverse(item)| item)
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
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
