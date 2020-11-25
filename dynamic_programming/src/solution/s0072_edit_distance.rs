#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
use std::i32::MAX;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (n, m) = (word1.len(), word2.len());

        if n * m == 0 {
            return (n + m) as i32;
        }

        let mut dp = vec![vec![MAX; m + 1]; n + 1];

        for i in 0..=n {
            dp[i][0] = i as i32;
        }

        for j in 0..=m {
            dp[0][j] = j as i32;
        }

        let wd1 = word1.chars().collect::<Vec<char>>();
        let wd2 = word2.chars().collect::<Vec<char>>();

        // dp compute
        for i in 1..=n {
            for j in 1..=m {
                let (left, down, mut left_down) =
                    (dp[i - 1][j] + 1, dp[i][j - 1] + 1, dp[i - 1][j - 1]);

                if wd1[i - 1] != wd2[j - 1] {
                    left_down += 1;
                }

                dp[i][j] = min(left, min(down, left_down));
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_72() {
        assert_eq!(
            Solution::min_distance("horse".to_owned(), "ros".to_owned()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_owned(), "execution".to_owned()),
            5
        );
    }
}
