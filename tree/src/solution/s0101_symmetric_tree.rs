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
        fn is_same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            match (p, q) {
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    p.val == q.val && is_same(&p.left, &q.right) && is_same(&p.right, &q.left)
                }
                (None, None) => true,
                _ => false,
            }
        }

        match root {
            Some(root) => is_same(&root.borrow().left, &root.borrow().right),
            None => true,
        }
    }

    pub fn is_symmetric_i(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        use std::collections::VecDeque;

        let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        deque.push_back(root.clone());
        deque.push_back(root);
        while let (Some(left), Some(right)) = (deque.pop_front(), deque.pop_front()) {
            if left.is_none() && right.is_none() {
                continue;
            } else if left.is_none() || right.is_none() {
                return false;
            } else if left.as_ref().unwrap().borrow().val != right.as_ref().unwrap().borrow().val {
                return false;
            }
            match (left, right) {
                (Some(l), Some(r)) => {
                    deque.push_back(l.borrow().left.clone());
                    deque.push_back(r.borrow().right.clone());
                    deque.push_back(l.borrow().right.clone());
                    deque.push_back(r.borrow().left.clone());
                },
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
