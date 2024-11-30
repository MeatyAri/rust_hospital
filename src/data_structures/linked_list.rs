use std::rc::Rc;
use std::cell::RefCell;

// Define a type alias for easier readability
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

// Node struct for LinkedList
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

// LinkedList struct
#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Link<T>,
}

impl<T> LinkedList<T> {
    // Create a new empty LinkedList
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a new element at the front
    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    // Remove and return the front element
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = Rc::try_unwrap(node).ok().expect("Multiple references exist");
            self.head = node.borrow_mut().next.take();
            node.into_inner().value
        })
    }

    // Check if the LinkedList is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Iterate over the LinkedList
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.clone(),
        }
    }
}

// Iterator for the LinkedList
pub struct LinkedListIterator<T> {
    current: Link<T>,
}

impl<T> Iterator for LinkedListIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let node_ref = node.borrow();
            self.current = node_ref.next.clone();
            node_ref.value.clone()
        })
    }
}

// fn main() {
//     let mut list = LinkedList::new();

//     list.push_front(1);
//     list.push_front(2);
//     list.push_front(3);

//     println!("Iterating through LinkedList:");
//     for value in list.iter() {
//         println!("{}", value);
//     }

//     println!("Popping front element: {:?}", list.pop_front());
//     println!("List after pop: {:?}", list);
// }
