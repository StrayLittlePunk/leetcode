#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // O(n) O(n)
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }
        let (mut ans, mut dp, chs) = (0, vec![0; s.len()], s.chars().collect::<Vec<char>>());

        for i in 1..chs.len() {
            if chs[i] == ')' {
                if chs[i - 1] == '(' {
                    dp[i] = (if i >= 2 { dp[i - 2] } else { 0 }) + 2;
                } else if i as i32 - dp[i - 1] > 0 && chs[i - dp[i - 1] as usize - 1] == '(' {
                    dp[i] = dp[i - 1]
                        + (if i as i32 - dp[i - 1] >= 2 {
                            dp[i - dp[i - 1] as usize - 2]
                        } else {
                            0
                        })
                        + 2;
                }
                ans = max(ans, dp[i]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_owned()), 2);
        assert_eq!(Solution::longest_valid_parentheses("()((())".to_owned()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()(())".to_owned()), 6);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);
    }
}
