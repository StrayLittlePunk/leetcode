#![allow(unused)]
pub struct Solution {}

// google interview
use std::cmp::min;
use std::i32::MAX;
impl Solution {
    pub fn num_squares_dp(n: i32) -> i32 {
        let mut dp = vec![MAX; n as usize + 1];
        // bottom case
        dp[0] = 0;

        // pre-calculate the square numbers
        let max_sqrt = f32::sqrt(n as f32) as usize + 1;
        let mut squares = vec![];
        for i in 0..max_sqrt {
            squares.push((i * i) as usize);
        }

        for i in 1..=n as usize {
            for s in 1..max_sqrt {
                if i < squares[s] {
                    break;
                }
                dp[i] = min(dp[i], dp[i - squares[s]] + 1);
            }
        }

        dp[n as usize]
    }

    // Mathematics solution
    pub fn num_squares(mut n: i32) -> i32 {
        while n % 4 == 0 {
            n /= 4;
        }

        if n % 8 == 7 {
            return 4;
        }

        if Self::is_square(n) {
            return 1;
        }

        let mut i = 1;
        while i * i <= n {
            if Self::is_square(n - i * i) {
                return 2;
            }

            i += 1;
        }

        return 3;
    }

    fn is_square(n: i32) -> bool {
        let sq = f32::sqrt(n as f32) as i32;
        return n == sq * sq;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_279() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
