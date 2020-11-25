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
    // Time O(N) Space O(log N)
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn is_valid_part(node: Option<Rc<RefCell<TreeNode>>>, val: i32, count: &mut i32) -> bool {
            match node {
                // considered a valid subtree
                None => true,
                Some(n) => {
                    // check if node.left and node.right are univalue subtrees of value node.val
                    // note that || short circuits but | does not - both sides of the or get evaluated
                    // with | so we explore all possible routes
                    if !is_valid_part(n.borrow().left.clone(), n.borrow().val, count)
                        | !is_valid_part(n.borrow().right.clone(), n.borrow().val, count)
                    {
                        return false;
                    }
                    // if it passed the last step then this a valid subtree - increment
                    *count += 1;

                    // at this point we know that this node is a univalue subtree of value node.val
                    // pass a boolean indicating if this is a valid subtree for the parent node
                    n.borrow().val == val
                }
            }
        }

        let mut count = 0;
        is_valid_part(root, 0, &mut count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_250() {
        assert_eq!(
            Solution::count_unival_subtrees(to_tree(vec![
                Some(5),
                Some(1),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(5)
            ])),
            4
        );
        assert_eq!(Solution::count_unival_subtrees(to_tree(vec![])), 0);
        assert_eq!(
            Solution::count_unival_subtrees(to_tree(vec![
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(5)
            ])),
            6
        );
    }
}
