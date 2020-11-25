#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
      if num <= 0 {
        return false;
      }

      for i  in 2..6 {
        while num % i == 0 {
          num /= i;
        }
      }
      num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_263() {
        assert_eq!(
            Solution::is_ugly(6),
            true
        );
        assert_eq!(
            Solution::is_ugly(8),
            true
        );
        assert_eq!(
            Solution::is_ugly(14),
            false
        );
    }
}
