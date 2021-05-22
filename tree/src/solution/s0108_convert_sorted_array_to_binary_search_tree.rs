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
    // Time O(N) Space O(logN)
    //  如果数据的个数是奇数，则中间那个数据就是这群数据的中位数；如果数据的个数是偶数，
    //  则中间那2个数据的算术平均值就是这群数据的中位数
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(nums: &Vec<i32>, l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if l >= r {
                return None;
            }

            let mid = l + (r - l) / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: helper(nums, l, mid),
                right: helper(nums, mid + 1, r),
            })))
        }

        helper(&nums, 0, nums.len())
    }
    pub fn sorted_array_to_bst_slice(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::helper(&nums)
    }
    fn helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }

        let median = nums.len() / 2;
        let left = Self::helper(&nums[0..median]);
        let right = Self::helper(&nums[median + 1..nums.len()]);

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[median],
            left,
            right,
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_108() {
        assert_eq!(
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
            to_tree(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)])
        );
    }
}
