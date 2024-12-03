use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use crate::db::entities::UniqueAttribute;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeNode<T> {
    pub value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Debug> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_value: T) {
        if new_value < self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(new_value),
                None => self.left = Some(Box::new(TreeNode::new(new_value))),
            }
        } else if new_value > self.value {
            match self.right {
                Some(ref mut right_child) => right_child.insert(new_value),
                None => self.right = Some(Box::new(TreeNode::new(new_value))),
            }
        }
    }

    pub fn contains(&self, target: T) -> bool {
        if target == self.value {
            true
        } else if target < self.value {
            match self.left {
                Some(ref left_child) => left_child.contains(target),
                None => false,
            }
        } else {
            match self.right {
                Some(ref right_child) => right_child.contains(target),
                None => false,
            }
        }
    }

    pub fn in_order_traversal(&self) {
        if let Some(ref left_child) = self.left {
            left_child.in_order_traversal();
        }
        println!("{:?}", self.value);
        if let Some(ref right_child) = self.right {
            right_child.in_order_traversal();
        }
    }

    pub fn get_by_uniq_attr(&self, uniq_attr: String) -> Option<&T>
    where
        T: UniqueAttribute,
    {
        if self.value.uattr() == uniq_attr {
            Some(&self.value)
        } else if uniq_attr < self.value.uattr() {
            match self.left {
                Some(ref left_child) => left_child.get_by_uniq_attr(uniq_attr),
                None => None,
            }
        } else {
            match self.right {
                Some(ref right_child) => right_child.get_by_uniq_attr(uniq_attr),
                None => None,
            }
        }
    }

    pub fn max(&self) -> T
    where
        T: Clone,
    {
        match self.right {
            Some(ref right_child) => right_child.max(),
            None => self.value.clone(),
        }
    }
}

// fn main() {
//     let mut root = TreeNode::new(10);
//     root.insert(5);
//     root.insert(15);
//     root.insert(3);
//     root.insert(7);
//     root.insert(12);
//     root.insert(18);

//     println!("Tree in-order traversal:");
//     root.in_order_traversal();

//     println!("Contains 7? {}", root.contains(7));  // true
//     println!("Contains 6? {}", root.contains(6));  // false
// }
