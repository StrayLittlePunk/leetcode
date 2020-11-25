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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {

        fn helper(node: Option<Rc<RefCell<TreeNode>>>, level: i32, res: &mut Vec<i32>) {
            match node {
                None => return,
                Some(n) => {
                    if level as usize == res.len() {
                        res.push(n.borrow().val);
                    }
                    if n.borrow().right.is_some() {
                      helper(n.borrow().right.clone(), level + 1, res);
                    }
                    if n.borrow().left.is_some() {
                      helper(n.borrow().left.clone(), level + 1, res);
                    }
                }
            }
        }

        let mut res: Vec<i32> = vec![];
        helper(root, 0, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_199() {
        // Input: [1,2,3,null,5,null,4]
        // Output: [1, 3, 4]
        // Explanation:
        //
        //    1            <---
        //  /   \
        // 2     3         <---
        //  \     \
        //   5     4       <---
        assert_eq!(
            Solution::right_side_view(to_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(5),
                None,
                Some(4)
            ])),
            vec![1, 3, 4]
        );
    }
}
