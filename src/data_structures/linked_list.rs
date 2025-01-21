use serde::{Serialize, Deserialize};
use std::fmt::Debug;

use crate::db::entities::UniqueAttribute;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    pub length: usize,
}

impl<T: Debug> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None, length: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn insert(&mut self, value: T) {
        self.push_front(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            self.length -= 1;
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

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }

        let mut current = self.head.as_ref();        
        for _ in 0..index {
            current = current.unwrap().next.as_ref();
        }
        current.map(|node| &node.value)
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

    pub fn remove_last_node(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            self.length -= 1;
            return self.head.take().map(|node| node.value);
        }

        let mut second_last = &mut self.head;
        while let Some(_node) = second_last.as_mut().unwrap().next.as_mut().unwrap().next.as_mut() {
            second_last = &mut second_last.as_mut().unwrap().next;
        }
        self.length -= 1;
        
        second_last.as_mut().unwrap().next.take().map(|node| node.value)
    }

    pub fn remove_by_uniq_attr(&mut self, uniq_attr: String) -> bool
    where
        T: UniqueAttribute + Clone,
    {
        let mut current = &mut self.head;
        while let Some(node) = current.as_mut() {
            if node.value.uattr() == uniq_attr {
                if let Some(next_node) = node.next.as_ref() {
                    node.value = next_node.value.clone();
                    node.next = next_node.next.clone();
                    self.length -= 1;
                } else {
                    // this function will adjust the length so no need for "self.length -= 1;"
                    self.remove_last_node();
                }
                return true;
            }

            current = &mut node.next;
        }
        false
    }

    pub fn iter(&self) -> LinkedListIter<'_, T> {
        LinkedListIter {
            current: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> LinkedListIterMut<'_, T> {
        LinkedListIterMut {
            current: self.head.as_deref_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        self.head = prev;
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            if &node.value == value {
                return true;
            }
            current = &node.next;
        }
        false
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

pub struct LinkedListIterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.value
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    struct TestStruct {
        id: u32,
        name: String,
    }

    impl UniqueAttribute for TestStruct {
        fn uattr(&self) -> String {
            self.id.to_string()
        }
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        assert_eq!(list.len(), 1);
        assert_eq!(list.head.as_ref().unwrap().value.id, 1);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        let popped = list.pop();
        assert_eq!(popped.unwrap().id, 1);
        assert!(list.is_empty());
    }

    #[test]
    fn test_get_by_index() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 0, name: "Alice".to_string() });
        list.push_front(TestStruct { id: 1, name: "Bob".to_string() });
        let node = list.get_by_index(0).unwrap();
        assert_eq!(node.id, 1);
        let node = list.get_by_index(1).unwrap();
        assert_eq!(node.id, 0)
    }

    #[test]
    fn test_get_by_uniq_attr() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        list.push_front(TestStruct { id: 2, name: "Bob".to_string() });
        let node = list.get_by_uniq_attr("1".to_string()).unwrap();
        assert_eq!(node.id, 1);
    }

    #[test]
    fn test_remove_last_node() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        list.push_front(TestStruct { id: 2, name: "Bob".to_string() });
        let removed = list.remove_last_node();
        assert_eq!(removed.unwrap().id, 1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.head.as_ref().unwrap().value.id, 2);
    }

    #[test]
    fn test_remove_by_uniq_attr() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        list.push_front(TestStruct { id: 2, name: "Bob".to_string() });
        let removed = list.remove_by_uniq_attr("1".to_string());
        assert!(removed);
        assert_eq!(list.len(), 1);
        assert_eq!(list.head.as_ref().unwrap().value.id, 2);
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        list.push_front(TestStruct { id: 2, name: "Bob".to_string() });
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().id, 2);
        assert_eq!(iter.next().unwrap().id, 1);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push_front(TestStruct { id: 1, name: "Alice".to_string() });
        list.push_front(TestStruct { id: 2, name: "Bob".to_string() });
        for node in list.iter_mut() {
            node.name.push_str(" Updated");
        }
        let mut iter = list.iter();
        assert_eq!(iter.next().unwrap().name, "Bob Updated");
        assert_eq!(iter.next().unwrap().name, "Alice Updated");
    }
}
