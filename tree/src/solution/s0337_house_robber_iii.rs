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
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    // Time O(2^N) Space O(N)
    // 隔代计算 ：如果计算root ， 然后计算 root.left.left root.left.right root.right.left
    // root.right.right。如果计算 root.left 则计算 root.left.left.left root.left.left.right
    // root.left.right.left root.left.right.right 加上得到的金额然后比较最大
    pub fn rob_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(r) => {
                let mut val = 0;
                if r.borrow().left.is_some() {
                    val += (Self::rob_1(r.borrow().left.as_ref().unwrap().borrow().left.clone())
                        + Self::rob_1(r.borrow().left.as_ref().unwrap().borrow().right.clone()));
                }
                if r.borrow().right.is_some() {
                    val += (Self::rob_1(r.borrow().right.as_ref().unwrap().borrow().left.clone())
                        + Self::rob_1(r.borrow().right.as_ref().unwrap().borrow().right.clone()));
                }

                max(
                    val + r.borrow().val,
                    (Self::rob_1(r.borrow().left.clone()) + Self::rob_1(r.borrow().right.clone())),
                )
            }
        }
    }
    // DP + Memo
    // Time O(N) Space O(N)
    pub fn rob_memo(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::HashMap;
        fn rob_sub(root: Option<Rc<RefCell<TreeNode>>>, map: &mut HashMap<usize, i32>) -> i32 {
            match root {
                None => 0,
                Some(r) => {
                    // if map.contains_key(&root) {
                    let key = Rc::into_raw(r.clone()) as usize;
                    if map.contains_key(&key) {
                        return *map.get(&key).unwrap();
                    }
                    let mut val = 0;
                    if r.borrow().left.is_some() {
                        val +=
                            (rob_sub(r.borrow().left.as_ref().unwrap().borrow().left.clone(), map)
                                + rob_sub(
                                    r.borrow().left.as_ref().unwrap().borrow().right.clone(),
                                    map,
                                ));
                    }
                    if r.borrow().right.is_some() {
                        val += (rob_sub(
                            r.borrow().right.as_ref().unwrap().borrow().left.clone(),
                            map,
                        ) + rob_sub(
                            r.borrow().right.as_ref().unwrap().borrow().right.clone(),
                            map,
                        ));
                    }

                    let res = max(
                        val + r.borrow().val,
                        (rob_sub(r.borrow().left.clone(), map)
                            + rob_sub(r.borrow().right.clone(), map)),
                    );

                    //map.insert(&root, res);
                    map.insert(key, res);
                    res
                }
            }
        }
        let mut map: HashMap<usize, i32> = HashMap::new();
        rob_sub(root, &mut map)
    }
    // DP + 状态压缩
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

      fn rob_sub(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
          None => (0, 0),
          Some(node) => {
            let left = rob_sub(node.borrow().left.clone());
            let right = rob_sub(node.borrow().right.clone());

            let l = max(left.0, left.1) + max(right.0, right.1);
            let r = node.borrow().val + left.0 + right.0;
            (l, r)
          }
        }

      }

      let (l, r) = rob_sub(root);
      max(l, r)

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_337() {
        assert_eq!(
            Solution::rob(to_tree(vec![
                Some(3),
                Some(4),
                Some(5),
                Some(1),
                Some(3),
                None,
                Some(1)
            ])),
            9
        );
        // Explanation: Maximum amount of money the thief can rob_1 = 4 + 5 = 9.
        assert_eq!(
            Solution::rob(to_tree(vec![
                Some(3),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
                Some(1)
            ])),
            7
        );
        assert_eq!(
            Solution::rob_memo(to_tree(vec![
                Some(3),
                Some(4),
                Some(5),
                Some(1),
                Some(3),
                None,
                Some(1)
            ])),
            9
        );
        // Explanation: Maximum amount of money the thief can rob_1 = 4 + 5 = 9.
        assert_eq!(
            Solution::rob_memo(to_tree(vec![
                Some(3),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
                Some(1)
            ])),
            7
        );
        // Explanation: Maximum amount of money the thief can rob_1 = 3 + 3 + 1 = 7.
        assert_eq!(
            Solution::rob_1(to_tree(vec![
                Some(3),
                Some(4),
                Some(5),
                Some(1),
                Some(3),
                None,
                Some(1)
            ])),
            9
        );
        // Explanation: Maximum amount of money the thief can rob_1 = 4 + 5 = 9.
        assert_eq!(
            Solution::rob_1(to_tree(vec![
                Some(3),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
                Some(1)
            ])),
            7
        );
        // Explanation: Maximum amount of money the thief can rob_1 = 3 + 3 + 1 = 7.
    }
}
