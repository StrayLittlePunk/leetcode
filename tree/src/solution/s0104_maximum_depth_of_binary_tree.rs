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
    // Time O(N) Space O(log N)
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                use std::collections::VecDeque;
                let mut deque: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
                let mut max_dep = std::i32::MIN;

                deque.push_back((Some(r), 1));
                while let Some((Some(node), height)) = deque.pop_front() {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        max_dep = max(max_dep, height);
                    } else {
                        if node.borrow().left.is_some() {
                            deque.push_back((node.borrow().left.clone(), height + 1));
                        }
                        if node.borrow().right.is_some() {
                            deque.push_back((node.borrow().right.clone(), height + 1));
                        }
                    }
                }
                max_dep
            }
        }
    }
     pub fn max_depth_preorder(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::cmp::max;
        let (mut stack, mut depth, mut max_depth) = (vec![], 0, 0);

        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                depth += 1;
                stack.push((depth, node.clone()));
                root = node.borrow().left.clone();
            }
            if let Some((dep, r)) = stack.pop() {
                depth = dep;
                max_depth = max(depth, max_depth);
                root = r.borrow().right.clone();
            }
        }
        max_depth
    }
    pub fn max_depth_r(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                let left_height = Self::max_depth_r(r.borrow().left.clone());
                let right_height = Self::max_depth_r(r.borrow().right.clone());
                max(left_height, right_height) + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_104() {
        assert_eq!(
            Solution::max_depth_r(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            3
        );
        assert_eq!(
            Solution::max_depth(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            3
        );
    }
}
