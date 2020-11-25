#![allow(unused)]
pub struct Solution {}
use std::collections::HashMap;
impl Solution {
  // O(n) O(n)
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut word_map = HashMap::new();
        let mut char_map = HashMap::new();
        let words = s.split(' ').collect::<Vec<&str>>();
        let chs = pattern.chars().collect::<Vec<char>>();

        if words.len() != chs.len() {
            return false;
        }

        for i in 0..words.len() {
            let ch = chs[i];
            let w = words[i];

            if !char_map.contains_key(&ch) {
                if word_map.contains_key(&w) {
                    return false;
                } else {
                    char_map.insert(ch, w);
                    word_map.insert(w, ch);
                }
            } else {
                if *char_map.get(&ch).unwrap() != w {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_290() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string(),),
            true
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string(),),
            false
        );
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string(),),
            false
        );
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string(),),
            false
        );
    }
}
