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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut ans = 0;
        if root.is_none() || k < 1 {
            return ans;
        }

        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut r = root.clone();
        let mut i = 0;
        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();
            if k != i {
                ans = r.as_ref().unwrap().borrow().val;
                i += 1;
            } else {
              break;
            }

            if let Some(node) = r {
                r = node.borrow().right.clone();
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_230() {
        assert_eq!(
            Solution::kth_smallest(to_tree(vec![Some(3), Some(1), Some(4), None, Some(2)]), 1),
            1
        );
        assert_eq!(
            Solution::kth_smallest(to_tree(vec![
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                None,
                Some(1)
            ]), 3),
            3
        );
    }
}
