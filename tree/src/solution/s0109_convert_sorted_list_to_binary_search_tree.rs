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
use crate::util::tree::{to_tree,TreeNode};
use crate::util::linked_list::{to_list, ListNode};

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    // Time O(N) Space O(logN)
     pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ptr = &head;
        let mut cnt = 0;
        while let Some(boxed) = ptr {
            cnt += 1;
            ptr = &boxed.next
        }
        let mut ptr = &head;
        Self::_list_to_bst(&mut ptr, cnt)
    }

    fn _list_to_bst(ptr: &mut &Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None;
        }
        let left = Self::_list_to_bst(ptr, len / 2);
        let boxed = ptr.as_ref().unwrap();
        let mut node = TreeNode::new(boxed.val);
        node.left = left;
        *ptr = &boxed.next;
        node.right = Self::_list_to_bst(ptr, len - len / 2 - 1);
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_109() {
        assert_eq!(
            Solution::sorted_list_to_bst(to_list(vec![-10, -3, 0, 5, 9])),
            to_tree(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)])
        );
    }
}
