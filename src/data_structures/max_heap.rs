const MAX_HEAP_SIZE: usize = 100;

#[derive(Debug)]
pub struct MaxHeap<T: Ord + Copy> {
    pub data: [Option<T>; MAX_HEAP_SIZE],
    size: usize,            // Current size of the heap
}

impl<T: Ord + Copy> MaxHeap<T> {
    // Create a new empty heap
    pub fn new() -> Self {
        Self {
            data: [None; MAX_HEAP_SIZE],
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
        let max_value = self.data[0];
        self.data[0] = self.data[self.size - 1]; // Move the last element to the root
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
}
