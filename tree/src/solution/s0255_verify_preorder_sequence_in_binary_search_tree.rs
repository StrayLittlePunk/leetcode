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
    pub fn verify_preorder(mut preorder: Vec<i32>) -> bool {
        let mut low = i32::MIN;
        let mut i = 0;
        let mut k = -1;
        while i < preorder.len() {
            if preorder[i] < low {
                return false;
            }
            while k >= 0 && preorder[i] > preorder[k as usize] {
                low = preorder[k as usize];
                k -= 1;
            }
            k += 1;
            preorder[k as usize] = preorder[i];
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_255() {
        assert_eq!(Solution::verify_preorder(vec![5, 2, 6, 1, 3]), false);
        assert_eq!(Solution::verify_preorder(vec![5, 2, 1, 3, 6]), true);
    }
}
