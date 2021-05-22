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
    pub fn has_path_sum_r(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    return sum == node.borrow().val;
                }
                Self::has_path_sum(node.borrow().left.clone(), sum - node.borrow().val)
                    || Self::has_path_sum(node.borrow().right.clone(), sum - node.borrow().val)
            }
        }
    }
    pub fn has_path_sum(mut root: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> bool {
        let mut stack = vec![];
        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                sum -= node.borrow().val;
                stack.push((node.clone(), sum));
                root = node.borrow().left.clone();
            }

            if let Some((node, rem)) = stack.pop() {
                // 回溯上一层的sum等于这一层的剩余值， 可变变量的后果。。
                sum = rem;
                // only if leaf node judge
                if node.borrow().left.is_none() && node.borrow().right.is_none() && rem == 0 {
                    return true;
                }
                if node.borrow().right.is_some() {
                    root = node.borrow().right.clone();
                }
            }
        }

        false
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
