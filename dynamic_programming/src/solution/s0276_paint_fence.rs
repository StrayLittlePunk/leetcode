#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        if n == 1 {
            return k;
        }

        if n == 2 {
            return k * k;
        }

        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;
        dp[1] = k;
        dp[2] = k * k;

        for i in 3..=n as usize {
            dp[i] = (dp[i - 1] + dp[i - 2]) * (k - 1);
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_276() {
        assert_eq!(Solution::num_ways(3, 2), 6);
    }
}
