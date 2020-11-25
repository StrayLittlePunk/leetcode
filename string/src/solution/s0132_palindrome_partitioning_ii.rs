#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
impl Solution {

    pub fn min_cut(s: String) -> i32 {
        if s.len() < 2 {
            return 0;
        }
        let mut dp = vec![];
        for i in 0..=s.len() {
            dp.push(i);
        }
        let chs = s.chars().collect::<Vec<char>>();
        let (mut start, mut end) = (0, 0);
        for mid in 1..s.len() {
            start = mid as i32;
            end = mid as i32;
            while start >= 0 && end < chs.len() as i32 && chs[start as usize] == chs[end as usize] {
                let new_cut = if start == 0 {
                    0
                } else {
                    dp[start as usize - 1] + 1
                };
                dp[end as usize] = min(dp[end as usize], new_cut);
                start -= 1;
                end += 1;
            }

            start = mid as i32 - 1;
            end = mid as i32;
            while start >= 0 && end < chs.len() as i32 && chs[start as usize] == chs[end as usize] {
                let new_cut = if start == 0 {
                    0
                } else {
                    dp[start as usize - 1] + 1
                };
                dp[end as usize] = min(dp[end as usize], new_cut);
                start -= 1;
                end += 1;
            }
        }

        dp[chs.len() - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_132() {
        assert_eq!(Solution::min_cut("ab".to_owned()), 1);
        assert_eq!(Solution::min_cut("a".to_owned()), 0);
        assert_eq!(Solution::min_cut("aab".to_owned()), 1);
    }
}
