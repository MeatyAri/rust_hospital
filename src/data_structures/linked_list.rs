use serde::{Serialize, Deserialize};
use std::fmt::Debug;

use crate::db::entities::UniqueAttribute;


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn insert(&mut self, value: T) {
        self.push_front(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.value
        })
    }

    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }

    pub fn get_by_uniq_attr(&mut self, uniq_attr: String) -> Option<&mut T>
    where
        T: UniqueAttribute,
    {
        let mut current = self.head.as_mut();
        while let Some(node) = current {
            if node.value.uattr() == uniq_attr {
                return Some(&mut node.value);
            }
            current = node.next.as_mut();
        }
        None
    }

    pub fn remove_last_node(&mut self) {
        if self.head.is_none() {
            return;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            self.head = None;
            return;
        }

        let mut second_last = &mut self.head;
        while let Some(node) = second_last.as_mut().unwrap().next.as_mut().unwrap().next.as_mut() {
            second_last = &mut node.next;
        }
    }

    pub fn remove_by_uniq_attr(&mut self, uniq_attr: String) -> bool
    where
        T: UniqueAttribute + Clone,
    {
        for node in self.head.iter_mut() {
            if node.value.uattr() == uniq_attr {
                if let Some(next_node) = node.next.as_ref() {
                    node.value = next_node.value.clone();
                    node.next = next_node.next.clone();
                } else {
                    self.remove_last_node();
                }
                return true;
            }
        }
        false
    }
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }
}

pub struct LinkedListIter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

// fn main() {
//     let mut list = LinkedList::new();
//     list.push_front(1);
//     list.push_front(2);
//     list.push_front(3);

//     list.display(); // Output: 3 -> 2 -> 1 -> None
// }
