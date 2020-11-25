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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        if root.is_none() {
            return res;
        }

        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut r = root.clone();
        while r.is_some() || !stack.is_empty() {
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            r = stack.pop();
            res.push(r.as_ref().unwrap().borrow().val);

            if let Some(node) = r {
                r = node.borrow().right.clone();
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_94() {
        assert_eq!(
            Solution::inorder_traversal(to_tree(vec![Some(1), None, Some(2), Some(3)])),
            vec![1, 3, 2]
        );
        assert_eq!(
            Solution::inorder_traversal(to_tree(vec![
                Some(1),
                Some(4),
                Some(2),
                Some(3),
                Some(5),
                None,
                Some(6)
            ])),
            vec![3, 4, 5, 1, 2, 6]
        );
    }
}
