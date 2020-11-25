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
    // Time O(N) Space O(1)
    pub fn is_valid_serialization(preorder: String) -> bool {
        if preorder.len() < 1 {
            return false;
        }

        let mut slots = 1;
        for s in preorder.split(',') {
            slots -= 1;

            if slots < 0 {
                return false;
            }

            if s != "#" {
                slots += 2;
            }
        }

        slots == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_331() {
        assert_eq!(
            Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()),
            true
        );
        assert_eq!(Solution::is_valid_serialization("1,#".to_string()), false);
        assert_eq!(
            Solution::is_valid_serialization("9,#,#,1".to_string()),
            false
        );
    }
}
