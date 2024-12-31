use std::fmt::Debug;
use serde::{Serialize, Deserialize};

use crate::db::entities::{Drug, UniqueAttribute};

use super::stack::Stack;

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

    fn find_min(&mut self) -> &mut TreeNode<T> {
        let mut current = self;
        while let Some(ref mut left_child) = current.left {
            current = left_child;
        }
        current
    }

    pub fn iter(&self) -> BstIterator<T> {
        BstIterator::new(Some(self))
    }

    fn extract_min_value(node: &mut TreeNode<T>) -> Option<T> {
        let mut current = node;
        while let Some(ref mut left) = current.left {
            current = left;
        }
        
        // Now current points to the node with minimum value
        if let Some(min_node) = current.left.take() {
            // Replace current node with its right child (if any)
            current.left = min_node.right;
            Some(min_node.value)
        } else {
            None
        }
    }
}

pub struct BstIterator<'a, T> {
    stack: Stack<&'a TreeNode<T>>,
}

impl<'a, T> BstIterator<'a, T> {
    fn new(root: Option<&'a TreeNode<T>>) -> Self {
        let mut stack = Stack::new();
        let mut current = root;
        while let Some(ref node) = current {
            stack.push(*node);
            current = node.left.as_deref();
        }
        BstIterator { stack }
    }
}

impl<'a, T> Iterator for BstIterator<'a, T>
where
    T: Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let value = &node.value;
            let mut current = &node.right;
            while let Some(ref node) = current {
                self.stack.push(node);
                current = &node.left;
            }
            Some(value)
        } else {
            None
        }
    }
}

impl TreeNode<Drug> {
    pub fn get_drug_by_id(&self, id: u32) -> Option<&Drug> {
        if self.value.id == id {
            Some(&self.value)
        } else if id < self.value.id {
            match self.left {
                Some(ref left_child) => left_child.get_drug_by_id(id),
                None => None,
            }
        } else {
            match self.right {
                Some(ref right_child) => right_child.get_drug_by_id(id),
                None => None,
            }
        }
    }

    pub fn get_drug_by_id_mut(&mut self, id: u32) -> Option<&mut Drug> {
        if self.value.id == id {
            Some(&mut self.value)
        } else if id < self.value.id {
            match self.left {
                Some(ref mut left_child) => left_child.get_drug_by_id_mut(id),
                None => None,
            }
        } else {
            match self.right {
                Some(ref mut right_child) => right_child.get_drug_by_id_mut(id),
                None => None,
            }
        }
    }

    pub fn get_drug_by_name(&self, name: String) -> Option<&Drug> {
        if self.value.name == name {
            Some(&self.value)
        } else {
            match self.left {
                Some(ref left_child) => left_child.get_drug_by_name(name),
                None => match self.right {
                    Some(ref right_child) => right_child.get_drug_by_name(name),
                    None => None,
                },
            }
        }
    }

    pub fn get_drug_by_name_mut(&mut self, name: String) -> Option<&mut Drug> {
        if self.value.name == name {
            Some(&mut self.value)
        } else {
            match self.left {
                Some(ref mut left_child) => left_child.get_drug_by_name_mut(name),
                None => match self.right {
                    Some(ref mut right_child) => right_child.get_drug_by_name_mut(name),
                    None => None,
                },
            }
        }
    }

    // pub fn remove_drug_by_id(&mut self, id: u32) -> Option<Box<TreeNode<Drug>>> {
    //     if id < self.value.id {
    //         if let Some(ref mut left_child) = self.left {
    //             self.left = left_child.remove_drug_by_id(id);
    //         }
    //     } else if id > self.value.id {
    //         if let Some(ref mut right_child) = self.right {
    //             self.right = right_child.remove_drug_by_id(id);
    //         }
    //     } else {
    //         // Node to be removed found
    //         if self.left.is_none() {
    //             // Case 1: No left child
    //             return self.right.take();
    //         } else if self.right.is_none() {
    //             // Case 2: No right child
    //             return self.left.take();
    //         } else {
    //             // Case 3: Two children
    //             let right_child = self.right.as_mut().unwrap();
    //             let min = right_child.find_min().clone();
    //             self.right = self.right.take().unwrap().remove_drug_by_id(min.value.id);
    //             self.value = min.value;
    //         }
    //     }
    //     None
    // }

    pub fn remove_drug_by_id(root: Option<Box<TreeNode<Drug>>>, id: u32) -> Option<Box<TreeNode<Drug>>> {
        if let Some(mut node) = root {
            if id < node.value.id {
                // Delete from left subtree
                node.left = Self::remove_drug_by_id(node.left, id);
                Some(node)
            } else if id > node.value.id {
                // Delete from right subtree
                node.right = Self::remove_drug_by_id(node.right, id);
                Some(node)
            } else {
                // This is the node to delete
                match (node.left.take(), node.right.take()) {
                    // No children or only right child
                    (None, right) => right,
                    // Only left child
                    (left, None) => left,
                    // Both children present
                    (left, right) => {
                        if let Some(mut right_tree) = right {
                            if let Some(successor_value) = Self::extract_min_value(&mut right_tree) {
                                node.value = successor_value;
                                node.right = Some(right_tree);
                                node.left = left;
                                return Some(node);
                            }
                        }
                        // This case should never happen if the tree is valid
                        left
                    }
                }
            }
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(18);

        assert!(root.contains(10));
        assert!(root.contains(5));
        assert!(root.contains(15));
        assert!(root.contains(3));
        assert!(root.contains(7));
        assert!(root.contains(12));
        assert!(root.contains(18));
        assert!(!root.contains(6));
    }

    #[test]
    fn test_in_order_traversal() {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(18);

        let mut result = Vec::new();
        root.in_order_traversal_collect(&mut result);
        assert_eq!(result, vec![3, 5, 7, 10, 12, 15, 18]);
    }

    #[test]
    fn test_max() {
        let mut root = TreeNode::new(10);
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        root.insert(12);
        root.insert(18);

        assert_eq!(root.max(), 18);
    }

    #[test]
    fn test_get_drug_by_name() {
        let mut root = TreeNode::new(Drug::new(1, "Aspirin".to_string(), 100.0, 5));
        root.insert(Drug::new(2, "Paracetamol".to_string(), 200.0, 10));
        root.insert(Drug::new(3, "Ibuprofen".to_string(), 150.0, 20));

        assert_eq!(root.get_drug_by_name("Aspirin".to_string()).unwrap().name, "Aspirin");
        assert_eq!(root.get_drug_by_name("Paracetamol".to_string()).unwrap().name, "Paracetamol");
        assert!(root.get_drug_by_name("NonExistent".to_string()).is_none());
    }

    #[test]
    fn test_remove_drug_by_id() {
        let mut root = TreeNode::new(Drug::new(1, "Aspirin".to_string(), 100.0, 5));
        root.insert(Drug::new(2, "Paracetamol".to_string(), 200.0, 10));
        root.insert(Drug::new(3, "Ibuprofen".to_string(), 150.0, 20));
        let mut wrapper = Some(Box::new(root));

        println!("{:?}", wrapper);
        wrapper = TreeNode::remove_drug_by_id(wrapper, 1);
        println!("{:?}", wrapper);
        wrapper = TreeNode::remove_drug_by_id(wrapper, 33);
        println!("{:?}", wrapper);
        wrapper = TreeNode::remove_drug_by_id(wrapper, 2);
        println!("{:?}", wrapper);
        wrapper = TreeNode::remove_drug_by_id(wrapper, 3);
        println!("{:?}", wrapper);

        if let Some(ref mut root) = wrapper {
            assert!(root.get_drug_by_name("Aspirin".to_string()).is_none());
        }
    }

    impl<T: Ord + Debug> TreeNode<T> {
        fn in_order_traversal_collect(&self, result: &mut Vec<T>)
        where
            T: Clone,
        {
            if let Some(ref left_child) = self.left {
                left_child.in_order_traversal_collect(result);
            }
            result.push(self.value.clone());
            if let Some(ref right_child) = self.right {
                right_child.in_order_traversal_collect(result);
            }
        }
    }
}
