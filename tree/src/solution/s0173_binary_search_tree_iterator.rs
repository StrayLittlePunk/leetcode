// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

#![allow(unused)]
use crate::util::tree::{to_tree, TreeNode};

use std::rc::Rc;
use std::cell::RefCell;
/*
 非递归中序遍历
 */
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {

    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut node = root;
        let mut stack = Vec::new();
        while let Some(inner) = node.clone() {
            stack.push(inner.clone());
            node = node.unwrap().borrow().left.clone();
        }
        BSTIterator{
            stack: stack,
        }
    }

    /** @return the next smallest number */
    pub fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();
        let res = node.borrow().val;
        let mut next = node.borrow().right.clone();
        while let Some(inner) = next.clone() {
            self.stack.push(inner.clone());
            next = next.unwrap().borrow().left.clone();
        }
        res
    }

    /** @return whether we have a next smallest number */
    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_173() {
        let root = to_tree(vec![
            Some(7),
            Some(3),
            Some(15),
            None,
            None,
            Some(9),
            Some(20),
        ]);
        let mut iterator = BSTIterator::new(root);
        assert_eq!(iterator.next(), 3); // return 3
        assert_eq!(iterator.next(), 7); // return 3
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 9); // return 3
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 15); // return 3
        assert_eq!(iterator.has_next(), true); // return true
        assert_eq!(iterator.next(), 20); // return 3
        assert_eq!(iterator.has_next(), false); // return true
    }
}
