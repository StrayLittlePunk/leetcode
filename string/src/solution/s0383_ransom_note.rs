#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
  // O(n) O(1)
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if magazine.len() < ransom_note.len() {
            return false;
        }

        let mut map = HashMap::new();
        for ch in magazine.chars() {
            let count = map.entry(ch).or_insert(0);
            *count += 1;
        }

        for ch in ransom_note.chars() {
            if !map.contains_key(&ch) {
                return false;
            }

            if *map.get(&ch).unwrap() == 0 {
                return false;
            } else if let Some(c) = map.get_mut(&ch) {
                *c -= 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {
        assert_eq!(
            Solution::can_construct("a".to_string(), "b".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "ab".to_string()),
            false
        );
        assert_eq!(
            Solution::can_construct("aa".to_string(), "aab".to_string()),
            true
        );
    }
}
