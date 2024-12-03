use std::fmt::Debug;
use std::iter::repeat;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{SeqAccess, Visitor};
use std::fmt;

use crate::db::entities::UniqueAttribute;

const MAX_HEAP_SIZE: usize = 100;

#[derive(Debug, Clone)]
pub struct MaxHeap<T: Ord + Clone> {
    pub data: [Option<T>; MAX_HEAP_SIZE],
    pub size: usize,            // Current size of the heap
}

impl<T: Ord + Clone + Debug> MaxHeap<T> {
    // Create a new empty heap
    pub fn new() -> Self {
        Self {
            data: repeat(None).take(MAX_HEAP_SIZE).collect::<Vec<_>>().try_into().unwrap(),
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.size >= self.data.len() {
            panic!("Heap overflow!"); // No dynamic resizing, fixed capacity
        }
        self.data[self.size] = Some(value);
        self.size += 1;
        self.bubble_up(self.size - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let max_value = self.data[0].clone();
        self.data[0] = self.data[self.size - 1].clone(); // Move the last element to the root
        self.data[self.size - 1] = None;
        self.size -= 1;
        self.bubble_down(0);
        max_value
    }

    pub fn peek(&self) -> Option<&T> {
        self.data[0].as_ref()
    }

    // Helper function to maintain the heap property after insertion
    pub fn bubble_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent_index = (index - 1) / 2;
            if self.data[index] > self.data[parent_index] {
                self.data.swap(index, parent_index);
                index = parent_index;
            } else {
                break;
            }
        }
    }

    // Helper function to maintain the heap property after extraction
    pub fn bubble_down(&mut self, mut index: usize) {
        loop {
            let left_child = 2 * index + 1;
            let right_child = 2 * index + 2;
            let mut largest = index;

            if left_child < self.size && self.data[left_child] > self.data[largest] {
                largest = left_child;
            }
            if right_child < self.size && self.data[right_child] > self.data[largest] {
                largest = right_child;
            }
            if largest == index {
                break;
            }
            self.data.swap(index, largest);
            index = largest;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get_by_uniq_attr(&self, uniq_attr: String) -> Option<&T>
    where 
        T: UniqueAttribute,
    {
        for i in 0..self.size {
            if self.data[i].as_ref().unwrap().uattr() == uniq_attr {
                return self.data[i].as_ref();
            }
        }
        None
    }
}


impl<T: Ord + Clone + Serialize> Serialize for MaxHeap<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.size))?;
        for item in &self.data[..self.size] {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<'de, T: Ord + Clone + Deserialize<'de> + Debug> Deserialize<'de> for MaxHeap<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MaxHeapVisitor<T: Ord + Clone>(std::marker::PhantomData<T>);

        impl<'de, T: Ord + Clone + Deserialize<'de> + Debug> Visitor<'de> for MaxHeapVisitor<T> {
            type Value = MaxHeap<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of options representing a MaxHeap")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut data: [Option<T>; 100] = repeat(None).take(MAX_HEAP_SIZE).collect::<Vec<_>>().try_into().unwrap();
                let mut size = 0;

                while let Some(value) = seq.next_element()? {
                    if size < MAX_HEAP_SIZE {
                        data[size] = value;
                        size += 1;
                    }
                }
                Ok(MaxHeap { data, size })
            }
        }

        deserializer.deserialize_seq(MaxHeapVisitor(std::marker::PhantomData))
    }
}