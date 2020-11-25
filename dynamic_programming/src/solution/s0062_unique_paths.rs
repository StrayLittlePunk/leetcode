#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(N * M) O(N * M)
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; n as usize]; m as usize];
        for i in 1..dp.len() {
            for j in 1..dp[0].len() {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m as usize - 1][n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_62() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
