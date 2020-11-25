#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
  // O(N) O(N) N is the length of s;
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = HashMap::new();

        for ch in s.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }

        for ch in t.chars() {
            if !map.contains_key(&ch) || *map.get(&ch).unwrap() == 0 {
                return false;
            } else if let Some(count) = map.get_mut(&ch) {
                *count -= 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string(),),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string(),),
            false
        );
    }
}
