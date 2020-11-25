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
    // Time O(N) Space O(1) Up to bottom
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let mut p = root.clone();

        while let Some(node) = p {
            // If the node has a left child
            if node.borrow().left.is_some() {
                // Find the rightmost node
                let mut rightmost = node.borrow().left.clone();

                while let Some(k) = rightmost {
                    if k.borrow().right.is_none() {
                        // rewire the connections
                        k.borrow_mut().right = node.borrow().right.clone();
                        let l = node.borrow().left.clone();
                        node.borrow_mut().right = l;
                        node.borrow_mut().left = None;

                        break;
                    }
                    rightmost = k.borrow().right.clone();
                }
            }

            // move on to the right side of the tree
            p = node.borrow().right.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_114() {
        let mut root = to_tree(vec![
            Some(1),
            Some(2),
            Some(5),
            Some(3),
            Some(4),
            None,
            Some(6),
        ]);
        Solution::flatten(&mut root);

        assert_eq!(
            root,
            to_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(3),
                None,
                Some(4),
                None,
                Some(5),
                None,
                Some(6)
            ])
        );
    }
}
