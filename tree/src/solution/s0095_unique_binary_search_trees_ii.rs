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
    // Time O(nGn) Space O(nGn) Gn is Catalan number
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }

        Self::helper(1, n)
    }

    fn helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut all_trees = vec![];
        if start > end {
            all_trees.push(None);
            return all_trees;
        }

        for i in start..end+1 {
            let left_trees = Self::helper(start, i - 1);
            let right_trees = Self::helper(i + 1, end);

            for l in left_trees.iter() {
                for r in right_trees.iter() {
                    let root = Some(Rc::new(RefCell::new(TreeNode {
                        val: i,
                        left: l.clone(),
                        right: r.clone(),
                    })));
                    all_trees.push(root);
                }
            }
        }

        all_trees
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_95() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                to_tree(vec![Some(1), None, Some(2), None, Some(3)]),
                to_tree(vec![Some(1), None, Some(3), Some(2)]),
                to_tree(vec![Some(2), Some(1), Some(3)]),
                to_tree(vec![Some(3), Some(1), None, None, Some(2)]),
                to_tree(vec![Some(3), Some(2), None, Some(1)]),
            ]
        );
    }
}
