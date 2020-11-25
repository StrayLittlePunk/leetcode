#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;
impl Solution {
  // O(N) O(1)
    pub fn first_uniq_char(s: String) -> i32 {
        if s.len() == 0 {
            return -1;
        }

        let mut map = HashMap::new();
        for ch in s.chars() {
            let counter = map.entry(ch).or_insert(0);
            *counter += 1;
        }

        let mut i = 0;
        for ch in s.chars() {
            if *map.get(&ch).unwrap() == 1 {
                return i;
            }
            i += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_387() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
