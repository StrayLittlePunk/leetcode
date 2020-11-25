#![allow(unused)]

pub struct Solution {}

// https://leetcode.com/problems/maximum-subarray/description/
//
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
              if nums.is_empty() {
        return 0;
      }
      let mut pre = 0;
      let mut max_ans = nums[0];
      for num in &nums {
        pre = max(pre + *num, *num);
        max_ans = max(max_ans, pre);
      }
      max_ans

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
      assert_eq!(Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
    }
}
