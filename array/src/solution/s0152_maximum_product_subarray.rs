#![allow(unused)]

pub struct Solution {}
// https://leetcode.com/problems/maximum-product-subarray/description/

use std::cmp::{max,min};
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
      if nums.is_empty() {
        return 0;
      }
      let mut maxf = nums[0];
      let mut minf = nums[0];
      let mut ans = nums[0];

      for i in 1..nums.len() {
        let mx = maxf;
        let mn = minf;
        maxf = max(mx * nums[i], max(nums[i], mn * nums[i]));
        minf = min(mn * nums[i], min(nums[i], mx * nums[i]));
        ans = max(maxf, ans);
      }
      ans
      
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_152() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
