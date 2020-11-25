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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    p.val == q.val && is_same(&p.left, &q.left) && is_same(&p.right, &q.right)
                }
                (None, None) => true,
                _ => false,
            }
        }

        is_same(&p, &q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_102() {
        assert_eq!(
            Solution::is_same_tree(
                to_tree(vec![Some(1), Some(2), Some(3)]),
                to_tree(vec![Some(1), Some(2), Some(3)])
            ),
            true
        );
        assert_eq!(
            Solution::is_same_tree(
                to_tree(vec![Some(1), Some(2)]),
                to_tree(vec![Some(1), None, Some(2)])
            ),
            false
        );
        assert_eq!(
            Solution::is_same_tree(
                to_tree(vec![Some(1), Some(2), Some(1)]),
                to_tree(vec![Some(1), Some(1), Some(2)])
            ),
            false
        );
    }
}
