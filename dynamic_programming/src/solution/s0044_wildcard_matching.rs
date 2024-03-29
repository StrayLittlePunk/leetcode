#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        //if p == 0 s != 0 false
        //if s==0 p!=0 all of p is *?
        for j in 1..n + 1 {
            if &p[j - 1..j] == "*" {
                dp[0][j] = true
            } else {
                break;
            }
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                let current_pattern = (&p[j - 1..j]).to_owned();
                let current_str = (&s[i - 1..i]).to_owned();
                if current_pattern != "*" {
                    dp[i][j] = dp[i - 1][j - 1]
                        && (current_str == current_pattern || current_pattern == "?");
                } else {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_44() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "*".to_owned()), true);
        assert_eq!(Solution::is_match("cb".to_owned(), "?a".to_owned()), false);
        assert_eq!(
            Solution::is_match("adceb".to_owned(), "*a*b".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()),
            false
        );
    }
}
