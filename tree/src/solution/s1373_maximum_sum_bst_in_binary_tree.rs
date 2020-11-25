
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
#[derive(Debug, Clone)]
pub struct Status {
    is_bst: bool,
    sum: i32,
    max_left: i32,
    min_right: i32,
}

use std::cell::RefCell;
use std::rc::Rc;
use crate::util::tree::{to_tree, TreeNode};

// amazon interview
impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) -> Status {
            match root {
                None => Status {
                    is_bst: true,
                    sum: 0,
                    max_left: std::i32::MIN,
                    min_right: std::i32::MAX,
                },
                Some(node) => {
                    let node = node.borrow();
                    let left = helper(&node.left, v);
                    let right = helper(&node.right, v);
                    let val = node.val;
                    let s = if val > left.max_left && val < right.min_right {
                        Status {
                            is_bst: left.is_bst && right.is_bst,
                            sum: val + left.sum + right.sum,
                            max_left: val.max(right.max_left),
                            min_right: val.min(left.min_right),
                        }
                    } else {
                        Status {
                            is_bst: false,
                            sum: 0,
                            max_left: std::i32::MIN,
                            min_right: std::i32::MAX,
                        }
                    };
                    if s.is_bst {
                        v.push(s.sum)
                    }
                    s
                }
            }
        }
        let mut v = vec![];
        helper(&root, &mut v);
        v.iter().fold(0, |acc, &x| if x > acc { x } else { acc })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1373() {

    }
}
