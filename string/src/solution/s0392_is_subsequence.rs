#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // O(26 * m + n) O(26 * m)
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let (n, m) = (s.len(), t.len());
        let (mut dp, s_char, t_char) = (
            vec![vec![0; m + 1]; n + 1],
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );

        for col in 1..=m {
            for row in 1..=n {
                if s_char[row - 1] == t_char[col - 1] {
                    dp[row][col] = dp[row - 1][col - 1] + 1;
                } else {
                    dp[row][col] = max(dp[row][col - 1], dp[row - 1][col]);
                }
            }

            if dp[n][col] == n {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392() {
        assert_eq!(
            Solution::is_subsequence("axc".to_owned(), "ahbgdc".to_owned()),
            false
        );
        assert_eq!(
            Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned()),
            true
        );
    }
}
