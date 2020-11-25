#![allow(unused)]
pub struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        let dict = word_dict.into_iter().collect::<HashSet<String>>();
        for i in 1..=n {
            for j in (0..i).rev() {
                if dp[j] && dict.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_139() {
        assert_eq!(
            Solution::word_break(
                "leetcode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned(),]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "applepenapple".to_owned(),
                vec!["apple".to_owned(), "pen".to_owned(),]
            ),
            true
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec![
                    "cats".to_owned(),
                    "dog".to_owned(),
                    "sand".to_owned(),
                    "and".to_owned(),
                    "cat".to_owned(),
                ]
            ),
            false
        );
    }
}
