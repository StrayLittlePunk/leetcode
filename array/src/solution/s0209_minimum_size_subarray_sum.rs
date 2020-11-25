#![allow(unused)]

pub struct Solution {}

// https://leetcode.com/problems/minimum-size-subarray-sum/description/

use std::cmp::min;
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut ans = i32::MAX;
        let mut start = 0;
        let mut end = 0;
        let mut sum = 0;

        while end <  nums.len() {
          sum += nums[end];
          while sum >= s {
            ans = min(ans, (end - start + 1) as i32);
            sum -= nums[start];
            start +=1;
          }
          end +=1;
        }
        let res = if ans == i32::MAX {
          0
        } else {
          ans
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }
}
