#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }

        let ch1 = s1.chars().collect::<Vec<char>>();
        let ch2 = s2.chars().collect::<Vec<char>>();
        let ch3 = s3.chars().collect::<Vec<char>>();

        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];

        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if i == 0 && j == 0 {
                    dp[i][j] = true;
                } else if i == 0 {
                    dp[i][j] = dp[i][j - 1] && ch2[j - 1] == ch3[i + j - 1];
                } else if j == 0 {
                    dp[i][j] = dp[i - 1][j] && ch1[i - 1] == ch3[i + j - 1];
                } else {
                    dp[i][j] = (dp[i - 1][j] && ch1[i - 1] == ch3[i + j - 1])
                        || (dp[i][j - 1] && ch2[j - 1] == ch3[i + j - 1]);
                }
            }
        }

        dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_97() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
        assert_eq!(
            Solution::is_interleave("".to_owned(), "".to_owned(), "".to_owned()),
            true
        );
    }
}
