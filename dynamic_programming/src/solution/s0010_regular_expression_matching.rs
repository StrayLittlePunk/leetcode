#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[s.len()][p.len()] = true;

        let chp = p.chars().collect::<Vec<char>>();
        let chs = s.chars().collect::<Vec<char>>();
        for i in (0..=s.len()).rev() {
            for j in (0..p.len()).rev() {
                let first_match = i < s.len() && (chp[j] == chs[i] || chp[j] == '.');

                if j + 1 < p.len() && chp[j + 1] == '*' {
                    dp[i][j] = dp[i][j + 2] || first_match && dp[i + 1][j];
                } else {
                    dp[i][j] = first_match && dp[i + 1][j + 1];
                }
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(Solution::is_match("ab".to_owned(), ".*".to_owned()), true);
        assert_eq!(
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()),
            false
        );
    }
}
