#![allow(unused)]
pub struct Solution {}

// amazon interview
impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        for i in 1..=f.min(target) {
            dp[i as usize] = 1;
        }
        let rem = 10i32.pow(9) + 7;
        for k in 1..d {
            let mut sum = ((target - f + 1).max(0)..=target)
                .into_iter()
                .fold(0, |acc, i| (acc + dp[i as usize]) % rem);
            for i in (k + 1..=target).rev() {
                sum = ((sum - dp[i as usize] + rem) % rem
                    + if i > f { dp[(i - f) as usize] } else { 0 })
                    % rem;
                dp[i as usize] = sum;
            }
            dp[k as usize] = 0;
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1155() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
    }
}
