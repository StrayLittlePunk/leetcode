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

// amazon interview

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let arr = Self::inorder(root);
        let (mut left, mut right) = (0, arr.len() - 1);
        while left < right {
            let sum = arr[left] + arr[right];
            if sum == k {
                return true;
            } else if sum < k {
                left += 1;
            } else {
                right -= 1;
            }
        }

        false
    }
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(r) => {
                let mut stack = vec![];
                let mut ret = vec![];
                let mut ro = Some(r);
                while ro.is_some() || !stack.is_empty() {
                    while let Some(node) = ro {
                        stack.push(node.clone());
                        ro = node.borrow().left.clone();
                    }

                    ro = stack.pop();
                    ret.push(ro.as_ref().unwrap().borrow().val);

                    if let Some(node) = ro {
                        ro = node.borrow().right.clone();
                    }
                }
                ret
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_653() {}
}
