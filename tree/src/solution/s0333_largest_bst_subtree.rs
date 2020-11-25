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
use std::i32::{MAX, MIN};
use std::cmp::{min, max};

impl Solution {
    // Time O(N) Space O(N)
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
      let ret = Self::largest_bst(root);
      ret.2
      
    }
    fn largest_bst(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
      match root {
        None => (i32::MAX, i32::MIN, 0),
        Some(node) => {
          let left = Self::largest_bst(node.borrow().left.clone());
          let right = Self::largest_bst(node.borrow().right.clone());

          if node.borrow().val > left.1 && node.borrow().val < right.0 {
            return (min(node.borrow().val, left.0), max(node.borrow().val, right.1), left.2 + right.2 + 1);
          } else {
            return (MIN, MAX, max(left.2, right.2));
          }
        },
      }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_333() {
        assert_eq!(
            Solution::largest_bst_subtree(to_tree(vec![
                Some(10),
                Some(5),
                Some(15),
                Some(1),
                Some(8),
                None,
                Some(7)
            ])),
            3
        );
    }
}
