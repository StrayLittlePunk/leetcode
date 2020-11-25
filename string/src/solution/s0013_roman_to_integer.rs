#![allow(unused)]
pub struct Solution {}
use std::collections::HashMap;

impl Solution {
  // O(1) O(1)
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let chs = s.chars().collect::<Vec<char>>();
        let mut total = *map.get(&chs[chs.len() - 1]).unwrap();
        for idx in (0..chs.len() - 1).rev() {
            if *map.get(&chs[idx]).unwrap() < *map.get(&chs[idx + 1]).unwrap() {
                total -= *map.get(&chs[idx]).unwrap();
            } else {
                total += *map.get(&chs[idx]).unwrap();
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_13() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
