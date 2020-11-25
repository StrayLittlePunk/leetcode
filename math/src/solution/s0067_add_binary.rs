#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
  //Time O(N) Space O(N)
    pub fn add_binary(a: String, b: String) -> String {
      let (m, n) = (a.len(), b.len());
      if m < n {
        return Self::add_binary(b, a);
      }

      let len = max(m, n);

      let mut carry = 0;
      let mut j = (n - 1) as i32;
      let b_str = b.as_str();
      let mut ans = "".to_string();

      for c in a.chars().rev() {
        if c == '1' {
          carry += 1;
        }
        if j >= 0 && &b_str[j as usize..(j+1) as usize] == "1" {
          carry += 1;
        }

        j -= 1;

        if carry % 2 == 1 {
          ans.push('1');
        } else {
          ans.push('0');
        }

        carry /= 2;
      }

      if carry == 1 {
        ans.push('1');
      }

      // reverse a string
      ans.chars().rev().collect()


    }
    pub fn add_binary_repeat(a: String, b: String) -> String {
      let (m, n) = (a.len(), b.len());
      if m < n {
        return Self::add_binary(b, a);
      }

      let mut carry = 0;
      let mut ans = "".to_string();
      
      // chain(std::iter::repeat('0') mean shorter will put 000...0 ulimited
      for (ac, bc) in a.chars().rev().zip(b.chars().rev().chain(std::iter::repeat('0'))) {
        if ac == '1' {
          carry += 1;
        }
        if bc == '1' {
          carry += 1;
        }

        if carry % 2 == 1 {
          ans.push('1');
        } else {
          ans.push('0');
        }

        carry /= 2;
      }

      if carry == 1 {
        ans.push('1');
      }

      ans.chars().rev().collect()

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(
            Solution::add_binary_repeat("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            Solution::add_binary_repeat("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
