// Define the structure of a Node in the linked list
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

// Define the structure of the Stack
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    // Create a new empty Stack
    pub fn new() -> Self {
        Stack { top: None }
    }

    // Push a value onto the Stack
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.top.take(),
        });
        self.top = Some(new_node);
    }

    // Pop a value off the Stack
    pub fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            Some(node) => {
                self.top = node.next;
                Some(node.value)
            }
            None => None,
        }
    }

    // Peek at the top value of the Stack without removing it
    pub fn peek(&self) -> Option<&T> {
        self.top.as_deref().map(|node| &node.value)
    }

    // Check if the Stack is empty
    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }
}

// fn main() {
//     let mut stack = Stack::new();

//     stack.push(1);
//     stack.push(2);
//     stack.push(3);

//     println!("Top value: {:?}", stack.peek()); // Output: Top value: Some(3)

//     while let Some(value) = stack.pop() {
//         println!("Popped: {}", value);
//     }

//     println!("Stack is empty: {}", stack.is_empty()); // Output: Stack is empty: true
// }
