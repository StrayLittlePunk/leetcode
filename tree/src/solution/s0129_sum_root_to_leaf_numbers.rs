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
    // Time O(N), Space O(N)
    // BFS + Queue
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                use std::collections::VecDeque;
                type Node = Option<Rc<RefCell<TreeNode>>>;
                let mut deque: VecDeque<(Node, i32)> = VecDeque::new();
                let val = r.borrow().val;
                let mut sum = 0;
                deque.push_back((Some(r), val));
                while let Some((Some(node), val)) = deque.pop_front() {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        sum += val;
                    } else {
                        if node.borrow().left.is_some() {
                            deque.push_back((
                                node.borrow().left.clone(),
                                val * 10 + node.borrow().left.as_ref().unwrap().borrow().val,
                            ));
                        }
                        if node.borrow().right.is_some() {
                            deque.push_back((
                                node.borrow().right.clone(),
                                val * 10 + node.borrow().right.as_ref().unwrap().borrow().val,
                            ));
                        }
                    }
                }
                sum
            }
        }
    }
    // DFS + Recursion
    pub fn sum_numbers_r(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        type Node = Option<Rc<RefCell<TreeNode>>>;
        fn sum_numbers_helper(root: &Node, sum: i32) -> i32 {
            match root {
                None => sum,
                Some(r) => {
                    if r.borrow().left.is_none() && r.borrow().right.is_none() {
                        sum * 10 + r.borrow().val
                    } else {
                        if r.borrow().left.is_some() && r.borrow().right.is_none() {
                            sum_numbers_helper(&r.borrow().left.clone(), sum * 10 + r.borrow().val)
                        } else if r.borrow().right.is_some() && r.borrow().left.is_none() {
                            sum_numbers_helper(&r.borrow().right.clone(), sum * 10 + r.borrow().val)
                        } else {
                            sum_numbers_helper(&r.borrow().left.clone(), sum * 10 + r.borrow().val)
                                + sum_numbers_helper(
                                    &r.borrow().right.clone(),
                                    sum * 10 + r.borrow().val,
                                )
                        }
                    }
                }
            }
        }
        sum_numbers_helper(&root, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_129() {
        assert_eq!(
            Solution::sum_numbers(to_tree(vec![Some(1), Some(2), Some(3)])),
            25
        );
        assert_eq!(
            Solution::sum_numbers(to_tree(vec![Some(4), Some(9), Some(0), Some(5), Some(1)])),
            1026
        );
        assert_eq!(
            Solution::sum_numbers_r(to_tree(vec![Some(1), Some(2), Some(3)])),
            25
        );
        assert_eq!(
            Solution::sum_numbers_r(to_tree(vec![Some(4), Some(9), Some(0), Some(5), Some(1)])),
            1026
        );
    }
}
