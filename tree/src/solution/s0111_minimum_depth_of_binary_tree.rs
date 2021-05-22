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

use std::cmp::min;

impl Solution {
    // Time O(N) Space O(log N)
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((root, 1));
        while let Some((Some(node), depth)) = queue.pop_front() {
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return depth;
            }

            if node.borrow().left.is_some() {
                queue.push_back((node.borrow().left.clone(), depth + 1));
            }

            if node.borrow().right.is_some() {
                queue.push_back((node.borrow().right.clone(), depth + 1));
            }
        }

        0
    }
    // Time O(N) Space O(log N)
    pub fn min_depth_r(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                if r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return 1;
                }
                let mut min_dep = std::i32::MAX;
                if r.borrow().left.is_some() {
                    min_dep = min(Self::min_depth_r(r.borrow().left.clone()), min_dep);
                }
                if r.borrow().right.is_some() {
                    min_dep = min(Self::min_depth_r(r.borrow().right.clone()), min_dep);
                }

                min_dep + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
        assert_eq!(
            Solution::min_depth_r(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            2
        );
        assert_eq!(
            Solution::min_depth(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            2
        );
    }
}
