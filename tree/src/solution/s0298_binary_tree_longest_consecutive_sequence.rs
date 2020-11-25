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

use std::cmp::max;
impl Solution {
    // Time O(N) Space O(N)
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root, &None, 0)
    }
    fn dfs(
        p: &Option<Rc<RefCell<TreeNode>>>,
        parent: &Option<Rc<RefCell<TreeNode>>>,
        mut length: i32,
    ) -> i32 {
        match (p, parent) {
            (None, _) => length,
            (Some(a), Some(b)) => {
                if a.borrow().val == b.borrow().val + 1 {
                    length += 1;
                    max(
                        length,
                        max(
                            Self::dfs(&a.borrow().left.clone(), &Some(a.clone()), length),
                            Self::dfs(&a.borrow().right.clone(), &Some(a.clone()), length),
                        ),
                    )
                } else {
                    length = 1;
                    max(
                        length,
                        max(
                            Self::dfs(&a.borrow().left.clone(), &Some(a.clone()), length),
                            Self::dfs(&a.borrow().right.clone(), &Some(a.clone()), length),
                        ),
                    )
                }
            }
            (Some(a), None) => {
                    length = 1;
                    max(
                        length,
                        max(
                            Self::dfs(&a.borrow().left.clone(), &Some(a.clone()), length),
                            Self::dfs(&a.borrow().right.clone(), &Some(a.clone()), length),
                        ),
                    )

            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_298() {
        assert_eq!(
            Solution::longest_consecutive(to_tree(vec![
                Some(1),
                None,
                Some(3),
                Some(2),
                Some(4),
                None,
                None,
                None,
                Some(5)
            ])),
            3
        );
        assert_eq!(
            Solution::longest_consecutive(to_tree(vec![
                Some(2),
                None,
                Some(3),
                Some(2),
                None,
                Some(1)
            ])),
            2
        );
    }
}
