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
pub struct Solution{}
use crate::util::tree::{to_tree, TreeNode};


use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
     // Time O(N) Space O(N)
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
      let mut res: Vec<i32> = vec![];
      if root.is_none() {
        return res;
      }

      let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
      let mut r = root.clone();
      while r.is_some() || !stack.is_empty() {
        while let Some(node) = r {
          res.push(node.borrow().val);
          stack.push(node.clone());
          r = node.borrow().left.clone();
        }
        r = stack.pop();
        if let Some(node) = r {
          r = node.borrow().right.clone();
        }
      }

      res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_144() {
      assert_eq!(
        Solution::preorder_traversal(to_tree(vec![Some(1), None, Some(2), Some(3)])),
        vec![1, 2, 3]);
      assert_eq!(
        Solution::preorder_traversal(
          to_tree(vec![Some(1), Some(4), Some(2), Some(3), Some(5), None, Some(6)])),
        vec![1, 4, 3, 5, 2, 6]);
    }
}
