#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n * m) O(n)
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        let (mut dp, mut prev, mut s_char, mut t_char) = (
            vec![0; n],
            1,
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );

        for i in (0..m).rev() {
            prev = 1;

            for j in (0..n).rev() {
                let old_dpj = dp[j];
                if s_char[i] == t_char[j] {
                    dp[j] += prev;
                }
                prev = old_dpj;
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_115() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned()),
            3
        );
        assert_eq!(
            Solution::num_distinct("babgbag".to_owned(), "bag".to_owned()),
            5
        );
    }
}
