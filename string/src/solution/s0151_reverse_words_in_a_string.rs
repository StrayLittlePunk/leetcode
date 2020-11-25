#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(N) O(N)
    pub fn reverse_words(s: String) -> String {
        let mut ret = s.split_whitespace().collect::<Vec<&str>>();
        ret.reverse();
        ret.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string(),),
            "blue is sky the".to_string(),
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string(),),
            "world hello".to_string(),
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_string(),),
            "example good a".to_string(),
        );
        assert_eq!(
            Solution::reverse_words("  Bob    Loves  Alice   ".to_string(),),
            "Alice Loves Bob".to_string(),
        );
    }
}
