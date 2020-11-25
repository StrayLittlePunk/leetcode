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

// uber interview
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut map = HashMap::new();
        let mut count = 0;
        Self::traverse(root, &mut map, sum, 0, &mut count);
        return count;
    }

    fn traverse(
        root: Option<Rc<RefCell<TreeNode>>>,
        map: &mut HashMap<i32, i32>,
        target: i32,
        mut cursum: i32,
        count: &mut i32,
    ) {
        match root {
            None => {
                return;
            }
            Some(node) => {
                cursum += node.borrow().val;

                if target == cursum {
                    *count += 1;
                }

                *count += map.get(&(cursum - target)).cloned().unwrap_or(0);

                *map.entry(cursum).or_insert(0) += 1;

                Self::traverse(node.borrow().left.clone(), map, target, cursum, count);
                Self::traverse(node.borrow().right.clone(), map, target, cursum, count);

                if let Some(x) = map.get_mut(&cursum) {
                    *x -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_437() {}
}
