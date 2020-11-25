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
    // Time O(H) Space O(1)
    pub fn closest_value(mut root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
      let mut closest = root.as_ref().unwrap().borrow().val;
      while let Some(node) = root {
        let v = node.borrow().val;
        if f64::abs(target - v as f64) < f64::abs(target - closest as f64) {
          closest = v;
        } 
        if target  < v as f64 {
          root = node.borrow().left.clone();
        } else {
          root = node.borrow().right.clone();
        }

      }
      closest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_270() {
        assert_eq!(
            Solution::closest_value(
                to_tree(vec![Some(4), Some(2), Some(5), Some(1), Some(3),]),
                3.714286
            ),
            4
        );
    }
}
