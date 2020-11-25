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
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
  // Time O(N), Space O(N)
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        match root {
            Some(r) => {
                use std::cmp::{max, min};
                let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
                let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
                let (mut min_col_idx, mut max_col_idx) = (0, 0);
                queue.push_back((r, 0));
                while !queue.is_empty() {
                    if let Some((node, column)) = queue.pop_front() {
                        min_col_idx = min(min_col_idx, column);
                        max_col_idx = max(max_col_idx, column);

                        let counter = map.entry(column).or_insert(vec![]);
                        counter.push(node.borrow().val);

                        if node.borrow().left.is_some() {
                            queue.push_back((node.borrow().left.clone().unwrap(), column - 1));
                        }
                        if node.borrow().right.is_some() {
                            queue.push_back((node.borrow().right.clone().unwrap(), column + 1));
                        }
                    }
                }

                let mut ans = vec![];
                let mut i = min_col_idx;
                while i <= max_col_idx {
                    if map.contains_key(&(i as i32)) {
                        ans.push(map.remove(&(i as i32)).unwrap());
                    }
                    i += 1;
                }
                ans
            }
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_314() {
        assert_eq!(
            Solution::vertical_order(to_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7),
            ]),),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );
        assert_eq!(
            Solution::vertical_order(to_tree(vec![
                Some(3),
                Some(9),
                Some(8),
                Some(4),
                Some(0),
                Some(1),
                Some(7),
            ]),),
            vec![vec![4], vec![9], vec![3, 0, 1], vec![8], vec![7]]
        );
        assert_eq!(
            Solution::vertical_order(to_tree(vec![
                Some(3),
                Some(9),
                Some(8),
                Some(4),
                Some(0),
                Some(1),
                Some(7),
                None,
                None,
                None,
                Some(2),
                Some(5)
            ]),),
            vec![vec![4], vec![9, 5], vec![3, 0, 1], vec![8, 2], vec![7]]
        );
    }
}
