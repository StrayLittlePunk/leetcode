#![allow(unused)]

pub struct Solution {}

// https://leetcode.com/problems/maximum-subarray/description/
//
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let mut min_sum = 0;
        let mut sum = 0;
        let mut ret = std::i32::MIN;
        for i in 0..nums.len() {
            sum += nums[i];
            // prefix_sum[j] - prefix_sum[i]
            ret = max(ret, sum - min_sum);
            min_sum = min(min_sum, sum);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
