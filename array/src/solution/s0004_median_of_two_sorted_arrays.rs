
#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/median-of-two-sorted-arrays/description/

use std::cmp::{max, min};
impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
      return Solution::find_median_sorted_arrays(nums2, nums1);
    }

    let m = nums1.len();
    let n = nums2.len();

    let mut left = 0;
    let mut right = m;
    let mut ansi: i32 = -1;
    let mut median1 = 0;
    let mut median2 = 0;

    while left <= right {
      let i = (left + right) / 2;
      let j = (m + n +1) / 2 - i;

      let nums_im1 = if i == 0 {
        std::i32::MIN
      } else {
        nums1[i -1]
      };
      let nums_i = if i == m {
        std::i32::MAX
      } else {
        nums1[i]
      };

      let nums_jm1 = if j == 0 {
        std::i32::MIN
      } else {
        nums2[j -1]
      };
      let nums_j = if j == n {
        std::i32::MAX
      } else {
        nums2[j]
      };
      
      if nums_im1 <= nums_j {
        ansi = i as i32;
        median1 = max(nums_im1, nums_jm1);
        median2 = min(nums_i, nums_j);
        left = i + 1;
      }else {
        right = i - 1;
      }

    }

    let res = if (m + n) % 2 == 0 {
      (median1 + median2) as f64 / 2.0 
    } else {
      median1 as f64
    };
    res
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(
          Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(
          Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
