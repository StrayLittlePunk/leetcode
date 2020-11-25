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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid_helper(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut i64) -> bool {
            match root {
                None => true,
                Some(node) => {
                    let l = is_valid_helper(node.borrow().left.clone(), prev);
                    // inorder
                    if l && node.borrow().val as i64 > *prev {
                        *prev = node.borrow().val as i64;
                    } else {
                        return false;
                    }
                    let r = is_valid_helper(node.borrow().right.clone(), prev);

                    l && r
                }
            }
        }
        let mut prev: i64 = std::i64::MIN;
        is_valid_helper(root, &mut prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_98() {
        assert_eq!(
            Solution::is_valid_bst(to_tree(vec![Some(1), Some(1)])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(to_tree(vec![Some(2), Some(1), Some(3)])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(to_tree(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ])),
            false
        );
    }
}
