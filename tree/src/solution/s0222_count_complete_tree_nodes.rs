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

use std::cmp::{max, min};
use std::i32::{MAX, MIN};
impl Solution {
    // Time O(d^2) Space O(1) d is the depth of tree
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let mut p = Some(node.clone());
                let mut d: u32 = 0;
                while let Some(n) = p {
                    p = n.borrow().left.clone();
                    d += 1;
                }

                d -= 1;
                if d == 0 {
                    return 1;
                }

                let (mut left, mut right) = (1, i32::pow(2, d) - 1);

                let mut pivot;
                while left <= right {
                    pivot = left + (right - left) / 2;
                    if Self::exists(pivot, d, Some(node.clone())) {
                        left = pivot + 1;
                    } else {
                        right = pivot - 1;
                    }
                }

                i32::pow(2, d) - 1 + left
            }
        }
    }

    fn exists(idx: i32, mut d: u32, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (mut left, mut right) = (0, i32::pow(2, d) - 1);
        let mut pivot;

        let mut p = root;

        d += 1;
        while let Some(node) = p {
            pivot = left + (right - left) / 2;
            d -= 1;
            if idx <= pivot {
                p = node.borrow().left.clone();
                right = pivot;
            } else {
                p = node.borrow().right.clone();
                left = pivot + 1;
            }
        }

        d == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_222() {
        assert_eq!(
            Solution::count_nodes(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
            ])),
            6
        );
    }
}
