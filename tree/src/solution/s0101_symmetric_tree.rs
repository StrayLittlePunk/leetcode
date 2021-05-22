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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(root.clone(), root)
    }
    fn is_mirror(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some(node1), Some(node2)) => {
                node1.borrow().val == node2.borrow().val
                    && Self::is_mirror(node1.borrow().right.clone(), node2.borrow().left.clone())
                    && Self::is_mirror(node1.borrow().left.clone(), node2.borrow().right.clone())
            }
        }
    }
    pub fn is_symmetric_i(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_front(root.clone());
        queue.push_front(root);
        while let (Some(node1), Some(node2)) = (queue.pop_back(), queue.pop_back()) {
            if node1.is_none() && node2.is_none() {
                continue;
            } else if node1.is_none() || node2.is_none() {
                return false;
            } else if node1.as_ref().unwrap().borrow().val != node2.as_ref().unwrap().borrow().val {
                return false;
            }

            match (node1, node2) {
                (Some(l), Some(r)) => {
                    queue.push_front(l.borrow().left.clone());
                    queue.push_front(r.borrow().right.clone());
                    queue.push_front(l.borrow().right.clone());
                    queue.push_front(r.borrow().left.clone());
                }
                (_, _) => {
                    continue;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_101() {
        assert_eq!(
            Solution::is_symmetric_i(to_tree(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(4),
                Some(4),
                Some(3)
            ])),
            true
        );
        assert_eq!(
            Solution::is_symmetric_i(to_tree(vec![
                Some(1),
                Some(2),
                Some(2),
                None,
                Some(3),
                None,
                Some(3)
            ])),
            false
        );
    }
}
