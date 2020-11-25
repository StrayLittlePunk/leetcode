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

use std::cmp::min;

impl Solution {
    // Time O(N) Space O(log N)
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                use std::collections::VecDeque;
                let mut deque: VecDeque<(Option<Rc<RefCell<TreeNode>>>, i32)> = VecDeque::new();
                let mut min_dep = 0;

                deque.push_back((Some(r), 1));
                while let Some((Some(node), height)) = deque.pop_front() {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        min_dep =  height;
                        break;
                    } else {
                        if node.borrow().left.is_some() {
                            deque.push_back((node.borrow().left.clone(), height + 1));
                        }
                        if node.borrow().right.is_some() {
                            deque.push_back((node.borrow().right.clone(), height + 1));
                        }
                    }
                }
                min_dep
            }
        }
    }
    // Time O(N) Space O(log N)
    pub fn min_depth_r(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                if r.borrow().left.is_none() && r.borrow().right.is_none() {
                    return 1;
                }
                let mut min_dep = std::i32::MAX;
                if r.borrow().left.is_some() {
                    min_dep = min(Self::min_depth_r(r.borrow().left.clone()), min_dep);
                }
                if r.borrow().right.is_some() {
                    min_dep = min(Self::min_depth_r(r.borrow().right.clone()), min_dep);
                }

                min_dep + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_111() {
        assert_eq!(
            Solution::min_depth_r(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            2
        );
        assert_eq!(
            Solution::min_depth(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            2
        );
    }
}
