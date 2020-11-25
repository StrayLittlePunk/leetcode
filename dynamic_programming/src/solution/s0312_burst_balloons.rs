#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() + 2;
        let mut new_nums = vec![0; n];

        for i in 0..nums.len() {
            new_nums[i + 1] = nums[i];
        }

        new_nums[n - 1] = 1;
        new_nums[0] = 1;

        // dp will store the results of our calls

        let mut dp = vec![vec![0; n]; n];

        // iterate over dp and incrementally build up to dp[0][n-1]
        for left in (0..(n - 1)).rev() {
            for right in (left + 2)..n {
                for i in (left + 1)..right {
                    // same formula to get the best score from (left, right) as before
                    dp[left][right] = max(
                        dp[left][right],
                        new_nums[left] * new_nums[i] * new_nums[right] + dp[left][i] + dp[i][right],
                    );
                }
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_312() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}
