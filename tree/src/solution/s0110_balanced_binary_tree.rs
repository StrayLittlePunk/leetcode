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

use std::cmp::max;

impl Solution {
    // Time O(N) Space O(N)

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_helper(root).1
    }

    // Return whether or not the tree at root is balanced while also storing
    // the tree's height in a reference variable.
    fn is_balanced_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        match root {
            // An empty tree is balanced and has height = -1
            None => (-1, true),
            Some(r) => {
                // Check subtrees to see if they are balanced. j
                let (left_height, lb) = Self::is_balanced_helper(r.borrow().left.clone());
                if !lb {
                    return (-1, false);
                }
                let (right_height, rb) = Self::is_balanced_helper(r.borrow().right.clone());
                if !rb {
                    return (-1, false);
                }

                // Use the height obtained from the recursive calls to
                // determine if the current node is also balanced.

                if (left_height - right_height).abs() < 2 {
                    return (max(left_height, right_height) + 1, true);
                }

                (-1, false)
            }
        }
    }
    pub fn is_balanced_r(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs_height(root) != -1
    }

    fn dfs_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_height = Self::dfs_height(node.borrow().left.clone());
                if left_height == -1 {
                    return -1;
                }
                let right_height = Self::dfs_height(node.borrow().right.clone());
                if right_height == -1 {
                    return -1;
                }
                if (left_height - right_height).abs() > 1 {
                    return -1;
                }

                max(left_height, right_height) + 1
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
            Solution::is_balanced(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            true
        );
        assert_eq!(
            Solution::is_balanced(to_tree(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3),
                None,
                None,
                Some(4),
                Some(4)
            ])),
            false
        );
    }
}
