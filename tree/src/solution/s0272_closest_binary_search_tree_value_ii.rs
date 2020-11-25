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

use std::collections::VecDeque;


impl Solution {
    // Time O(N) Space O(N)
    pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
      let mut queue: VecDeque<i32> = VecDeque::new();

      Self::inorder(root, &mut queue);
      while queue.len() > k as usize {
        if f64::abs(*queue.front().unwrap() as f64 - target) > f64::abs(*queue.back().unwrap() as f64 - target) {
          queue.pop_front();
        }else {
          queue.pop_back();
        }
      }

      Vec::from(queue)

    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, queue: &mut VecDeque<i32>) {
      match root {
        Some(node) => {
          Self::inorder(node.borrow().left.clone(), queue);
          queue.push_back(node.borrow().val);
          Self::inorder(node.borrow().right.clone(), queue);
        },
        None => {return;},
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_272() {
        assert_eq!(
            Solution::closest_k_values(
                to_tree(vec![Some(4), Some(2), Some(5), Some(1), Some(3),]),
                3.714286,
                2
            ),
            vec![3, 4]
        );
    }
}
