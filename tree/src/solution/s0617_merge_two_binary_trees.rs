
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn merge_trees(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (t1, t2) {
            (Some(node1), Some(node2)) => {
                Some(Rc::new(RefCell::new(TreeNode{val: node1.borrow().val + node2.borrow().val, 
                    left: Self::merge_trees(node1.borrow().left.clone(), node2.borrow().left.clone()), 
                    right: Self::merge_trees(node1.borrow().right.clone(), node2.borrow().right.clone())})))
            }, 
            (Some(node), None) | (None, Some(node)) => {
                Some(Rc::new(RefCell::new(TreeNode{val: node.borrow().val, 
                    left: Self::merge_trees(node.borrow().left.clone(), None), 
                    right: Self::merge_trees(node.borrow().right.clone(), None)})))
            },
            (None, None) => {
                None 
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_617() {

    }
}
