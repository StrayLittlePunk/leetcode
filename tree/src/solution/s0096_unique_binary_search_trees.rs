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
    //
    pub fn num_trees(n: i32) -> i32 {
        //Catalan number : C(0) = 1, C(n+1) = 2(2n+1)/ (n + 2) * C(n)
        let mut c: i64 = 1;
        for i in 0..n {
            c = c * (2 * (2 * i + 1)) as i64 / ((i + 2) as i64);
        }
        c as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_96() {
        assert_eq!(Solution::num_trees(3), 5);
    }
}
