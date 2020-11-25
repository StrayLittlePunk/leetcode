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
    // Time O(N) Space O(log N) ~ O(N)
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::cmp::max;
        fn height(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) -> i32 {
            match root {
                None => -1,
                Some(n) => {
                    let level = 1 + max(
                        height(n.borrow().left.clone(), res),
                        height(n.borrow().right.clone(), res),
                    );
                    if res.len() == level as usize {
                        res.push(vec![]);
                    }
                    res[level as usize].push(n.borrow().val);
                    level
                }
            }
        }

        let mut res: Vec<Vec<i32>> = vec![];
        height(root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_366() {
        // Normally:
        //                              1    (height: 0)
        //                             / \
        //               (height: 1)  2   3  (height: 1)
        //                           / \
        //              (height: 2) 4   5    (height: 2)
        //
        // Calculate Height from leafs:
        //                             1     (height: 2 - from the left branch because it's returning bigger height than the right)
        //                            / \
        //              (height: 1)  2   3   (height: 0 - started to calculate Height from the beginning as leafs are null)
        //                          / \
        //             (height: 0) 4   5     (height: 0)

        assert_eq!(
            Solution::find_leaves(to_tree(vec![Some(1), Some(2), Some(3), Some(4), Some(5)])),
            vec![vec![4, 5, 3], vec![2], vec![1]]
        );
    }
}
