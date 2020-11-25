#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut dp = vec![0; s.len() + 1];
        let s = s.as_str();
        dp[0] = 1;

        let ws = s.chars().collect::<Vec<char>>();
        dp[1] = if ws[0] == '0' { 0 } else { 1 };

        for i in 2..dp.len() {
            if ws[i - 1] != '0' {
                dp[i] += dp[i - 1];
            }
            let two_digit = *&s[i - 2..i].parse().unwrap_or(0);
            if two_digit >= 10 && two_digit <= 26 {
                dp[i] += dp[i - 2];
            }
        }

        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_91() {
        assert_eq!(Solution::num_decodings("12".to_owned()), 2);
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings("0".to_owned()), 0);
    }
}
