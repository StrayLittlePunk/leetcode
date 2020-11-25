#![allow(unused)]
pub struct Solution {}

// amazon interview
use std::cmp::{min, max};
use std::i32::MAX;
impl Solution {
    //dp[i][j]表示前i天完成前j项任务所需的最小难度,i、j都是从0开始数。
    // 给定一个数组，切分成d个子数组，求每个子数组的最大值的和最小 :q
    //
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (mut maxd, n) = (0, job_difficulty.len());
        if n < d as usize {
            return -1;
        }
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            dp[i] = max(dp[i+1], job_difficulty[i]);
        }
        
        for d in 2..=d as usize {
            for i in 0..=n - d as usize {
                maxd = 0;
                dp[i] = MAX;
                for j in i..=n - d as usize {
                    maxd = max(maxd, job_difficulty[j]);
                    dp[i] = min(dp[i], maxd + dp[j + 1]);
                }
            }
        }
        return dp[0];
    }
}

 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1335() {
    }
}
