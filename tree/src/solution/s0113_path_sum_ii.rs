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
    // Time O(N) Space O(N)
    // DFS + Stack
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut sums: Vec<Vec<i32>> = vec![];
        match root {
            None => sums,
            Some(r) => {
                let mut val_stack: Vec<Vec<i32>> = vec![];
                let mut node_stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
                val_stack.push(vec![0]);
                node_stack.push(Some(r));
                while let (Some(Some(node)), Some(mut vals)) = (node_stack.pop(), val_stack.pop()) {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        if node.borrow().val + vals.pop().unwrap() == sum {
                            vals.push(node.borrow().val);
                            sums.push(vals);
                        }
                    } else {
                        let cur_sum = vals.pop().unwrap() + node.borrow().val;
                        if node.borrow().left.is_some() {
                            // left always need to clone one time ... need to fix
                            let mut left = vals.clone();

                            left.push(node.borrow().val);
                            left.push(cur_sum);
                            val_stack.push(left);
                            node_stack.push(node.borrow().left.clone());
                        }
                        if node.borrow().right.is_some() {
                            vals.push(node.borrow().val);
                            vals.push(cur_sum);
                            val_stack.push(vals);
                            node_stack.push(node.borrow().right.clone());
                        }
                    }
                }

                sums
            }
        }
    }

    // BFS + Queue
    pub fn path_sum_bfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut sums: Vec<Vec<i32>> = vec![];
        match root {
            None => sums,
            Some(r) => {
                use std::collections::VecDeque;
                type Node = Option<Rc<RefCell<TreeNode>>>;
                let mut deque: VecDeque<(Node, i32, Vec<i32>)> = VecDeque::new();
                let init_val = r.borrow().val;
                deque.push_back((Some(r), init_val, vec![init_val]));
                while let Some((Some(cur), val, mut ls)) = deque.pop_front() {
                    if cur.borrow().left.is_none() && cur.borrow().right.is_none() && val == sum {
                        sums.push(ls);
                    } else {
                        if cur.borrow().left.is_some() {
                            let left_val = cur.borrow().left.as_ref().unwrap().borrow().val;
                            // left always need to clone one time ... need to fix
                            let mut left_vec = ls.clone();
                            left_vec.push(left_val);
                            /* print!("left: [");
                             * for v in &left_vec {
                             *   print!("{}, ", v);
                             * }
                             * println!("]"); */
                            deque.push_back((cur.borrow().left.clone(), val + left_val, left_vec));
                        }

                        if cur.borrow().right.is_some() {
                            let right_val = cur.borrow().right.as_ref().unwrap().borrow().val;
                            ls.push(right_val);
                            /* print!("right: [");
                             * for v in &ls {
                             *   print!("{}, ", v);
                             * }
                             * println!("]"); */
                            deque.push_back((
                                cur.borrow().right.clone(),
                                val + right_val,
                                ls
                            ));
                        }
                    }
                }

                sums
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112() {
        assert_eq!(
            //       5
            //      / \
            //     4   8
            //    /   / \
            //   11  13  4
            //  /  \    / \
            // 7    2  5   1
            Solution::path_sum(
                to_tree(vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    Some(5),
                    Some(1)
                ]),
                22
            ),
            vec![vec![5, 8, 4, 5], vec![5, 4, 11, 2]]
        );
        assert_eq!(
            //       5
            //      / \
            //     4   8
            //    /   / \
            //   11  13  4
            //  /  \    / \
            // 7    2  5   1
            Solution::path_sum_bfs(
                to_tree(vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    Some(5),
                    Some(1)
                ]),
                22
            ),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }
}
