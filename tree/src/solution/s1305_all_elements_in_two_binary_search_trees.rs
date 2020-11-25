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

// facebook interview
#![allow(unused)]
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let (mut vec1, mut vec2) = (vec![], vec![]);
        // translate tree to vector using inorder traversal
        Self::to_vec(root1, &mut vec1);
        Self::to_vec(root2, &mut vec2);
        // merget two sorted list
        Self::merge(vec1, vec2)
    }

    fn to_vec(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                Self::to_vec(node.borrow().left.clone(), ans);
                ans.push(node.borrow().val);
                Self::to_vec(node.borrow().right.clone(), ans);
            }
        }
    }

    fn merge(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
        if vec1.len() == 0 {
            return vec2;
        }
        if vec2.len() == 0 {
            return vec1;
        }
        let (mut ans, mut i, mut j) = (vec![], 0, 0);
        while i < vec1.len() && j < vec2.len() {
            if vec1[i] < vec2[j] {
                ans.push(vec1[i]);
                i += 1;
            } else {
                ans.push(vec2[j]);
                j += 1;
            }
        }
        // vec1 have more elements
        while i < vec1.len() {
            ans.push(vec1[i]);
            i += 1;
        }
        // vec2 have more elements
        while j < vec2.len() {
            ans.push(vec2[j]);
            j += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1305() {
    }
}
