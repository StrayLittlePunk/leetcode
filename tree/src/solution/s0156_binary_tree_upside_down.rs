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
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    // Time O(n) Space O(logn)
    pub fn upside_down_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                if node.borrow().left.is_none() {
                    return Some(node);
                }
                let new_root = Self::upside_down_binary_tree(node.borrow().left.clone());
                if let Some(left) = node.borrow().left.clone() {
                    left.borrow_mut().left = node.borrow().right.clone();
                    left.borrow_mut().right = Some(node.clone());
                }
                node.borrow_mut().left = None;
                node.borrow_mut().right = None;
                new_root
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_270() {
        assert_eq!(
            Solution::upside_down_binary_tree(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
            ])),
            to_tree(vec![
                Some(4),
                Some(5),
                Some(2),
                None,
                None,
                Some(3),
                Some(1)
            ])
        );
        assert_eq!(
            Solution::upside_down_binary_tree(to_tree(vec![Some(1)])),
            to_tree(vec![Some(1)])
        );
    }
}
