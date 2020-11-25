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
use std::i32::MIN;
use std::rc::Rc;

impl Solution {
    // Time O(N) Space O(log N)
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn path_sum_helper(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
            match root {
                None => 0,
                Some(r) => {
                    // 递归计算左右子节点的最大贡献值
                    // 只有在最大贡献值大于 0 时，才会选取对应子节点
                    let left = max(0, path_sum_helper(r.borrow().left.clone(), sum));
                    let right = max(0, path_sum_helper(r.borrow().right.clone(), sum));

                    // 节点的最大路径和取决于该节点的值与该节点的左右子节点的最大贡献值
                    let imr = r.borrow().val + left + right;

                    let ret = r.borrow().val + max(left, right);

                    // 更新答案
                    *sum = max(*sum, max(imr, ret));

                    // 返回节点的最大贡献值
                    ret
                }
            }
        }
        let mut sum = MIN;
        path_sum_helper(root, &mut sum);
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_124() {
        assert_eq!(
            Solution::max_path_sum(to_tree(vec![Some(1), Some(2), Some(3),])),
            6
        );
        assert_eq!(
            Solution::max_path_sum(to_tree(vec![
                Some(-10),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            42
        );
    }
}
