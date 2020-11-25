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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            None => false,
            Some(r) => {
                let mut val_stack: Vec<i32> = vec![];
                let mut node_stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
                val_stack.push(0);
                node_stack.push(Some(r));
                while let (Some(Some(node)), Some(val)) = (node_stack.pop(), val_stack.pop()) {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        if node.borrow().val + val == sum {
                            return true;
                        }
                    } else {
                        if node.borrow().left.is_some() {
                            val_stack.push(val + node.borrow().val);
                            node_stack.push(node.borrow().left.clone());
                        }
                        if node.borrow().right.is_some() {
                            val_stack.push(val + node.borrow().val);
                            node_stack.push(node.borrow().right.clone());
                        }
                    }
                }

                false
            }
        }
    }
    pub fn has_path_sum_r(root: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> bool {
        match root {
            None => false,
            Some(r) => {
                sum -= r.borrow().val;
                if r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return (sum == 0);
                }
                Self::has_path_sum_r(r.borrow().left.clone(), sum)
                    || Self::has_path_sum_r(r.borrow().right.clone(), sum)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(
            Solution::has_path_sum(
                to_tree(vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    None,
                    Some(1)
                ]),
                22
            ),
            true
        );
    }
}
