#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(1) 
    pub fn title_to_number(s: String) -> i32 {
      let mut ret = 0;
      for ch in s.chars() {
        ret *= 26;
        ret += (ch as u32 - 'A' as u32 + 1);
      }

      ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_171() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }
}
