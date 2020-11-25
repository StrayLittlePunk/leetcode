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

use std::collections::HashMap;

impl Solution {
    // Time O(N) Space O(N)
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::with_capacity(inorder.len());
        for i in 0..inorder.len() {
            map.insert(inorder[i], i);
        }
        let mut pre_idx = 0;

        fn helper(
            in_left: usize,
            in_right: usize,
            pre_idx: &mut usize,
            map: &HashMap<i32, usize>,
            preorder: &Vec<i32>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if in_left == in_right {
              return None;
            }
            let root_val = preorder[*pre_idx];
            let mut root = Rc::new(RefCell::new(TreeNode::new(root_val)));

            let idx = *map.get(&root_val).unwrap();

            *pre_idx += 1;
            root.borrow_mut().left = helper(in_left, idx, pre_idx, map, preorder);
            root.borrow_mut().right = helper(idx + 1, in_right, pre_idx, map, preorder);

            Some(root)
        }

        helper(0, inorder.len(), &mut pre_idx, &map, &preorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_105() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])
        );
    }
}
