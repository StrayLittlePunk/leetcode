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
    // Time O(N) Space O(N)
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      match root {
        None => vec![],
        Some(root) => {
          let mut stack1: Vec<Rc<RefCell<TreeNode>>> = vec![root];
          let mut stack2: Vec<Rc<RefCell<TreeNode>>> = vec![];
          while let Some(node) = stack1.pop() {
            if let Some(left) = node.borrow().left.clone() {
              stack1.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
              stack1.push(right);
            }
            stack2.push(node);
          }


          let mut res = vec![];
          while let Some(node) = stack2.pop() {
            res.push(node.borrow().val);
          }
          res
        }
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_145() {
        assert_eq!(
            Solution::postorder_traversal(to_tree(vec![Some(1), None, Some(2), Some(3)])),
            vec![3, 2, 1]
        );
        assert_eq!(
            Solution::postorder_traversal(to_tree(vec![])),
            vec![]
        );
        assert_eq!(
            Solution::postorder_traversal(to_tree(vec![Some(1)])),
            vec![1]
        );
        assert_eq!(
            Solution::postorder_traversal(to_tree(vec![Some(1), Some(2)])),
            vec![2, 1]
        );
        assert_eq!(
            Solution::postorder_traversal(to_tree(vec![Some(1), None, Some(2)])),
            vec![2, 1]
        );
        assert_eq!(
            Solution::postorder_traversal(to_tree(vec![
                Some(1),
                Some(4),
                Some(2),
                Some(3),
                Some(5),
                None,
                Some(6)
            ])),
            vec![3, 5, 4, 6, 2, 1]
        );
    }
}
