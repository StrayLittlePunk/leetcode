
#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/median-of-two-sorted-arrays/description/

use std::cmp::{max, min};
impl Solution {
  pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {

  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
          Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5), 
          vec![9, 8 , 6, 5, 3]);
        assert_eq!(
          Solution::max_number(vec![6, 7], vec![6, 0, 4], 5), 
          vec![6, 7, 6, 0, 4]);
        assert_eq!(
          Solution::max_number(vec![3, 9], vec![8, 9], 3), 
          vec![9, 8, 9]);
    }
}
