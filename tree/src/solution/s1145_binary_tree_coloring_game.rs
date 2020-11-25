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

// google interview
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn count(root: Option<Rc<RefCell<TreeNode>>>, con: &mut [i32], x: i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
             let (left, right) = (count(node.borrow().left.clone(), con, x), count(node.borrow().right.clone(), con, x));
            if node.borrow().val == x {
                con[0] = left;
                con[1] = right;
            }
            left + right + 1
                }
            }

        }

        let mut container = [0, 0];
        let ret = count(root, &mut container, x) / 2;
        use std::cmp::max;
        ret < max(max(container[0], container[1]), n - container[0] - container[1] - 1)

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1145() {

    }
}
