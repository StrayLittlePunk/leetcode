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

// microsoft interview
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{max, min};
impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
           Some(node) => Self::dfs(Some(node.clone()), node.borrow().val, node.borrow().val),
           None => 0,
        }
    }
    
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut mx: i32, mut mn: i32) -> i32 {
        match root {
            None => mx - mn,
            Some(node) => {
                mx = max(mx, node.borrow().val);
                 mn = min(mn, node.borrow().val);
               max(Self::dfs(node.borrow().left.clone(), mx, mn), Self::dfs(node.borrow().right.clone(), mx, mn))
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1026() {

    }
}
