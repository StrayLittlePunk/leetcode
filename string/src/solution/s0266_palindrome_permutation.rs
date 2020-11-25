#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    // O(N) O(1)
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut map = HashMap::new();

        s.chars().for_each(|ch| {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        });

        let cnt = map.values().fold(0, |acc, b| acc + b % 2);
        cnt <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(Solution::can_permute_palindrome("code".to_owned()), false);
        assert_eq!(Solution::can_permute_palindrome("carerac".to_owned()), true);
        assert_eq!(Solution::can_permute_palindrome("aab".to_owned()), true);
    }
}
