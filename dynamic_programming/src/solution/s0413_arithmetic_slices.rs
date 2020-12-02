#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut dp = vec![0;a.len()];
        let mut sum = 0;
        for i in 2..dp.len() {
            if a[i] - a[i - 1] == a[i - 1] - a[i - 2] {
                dp[i] = 1 + dp[i - 1];
                sum += dp[i];
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_413() {}
}
