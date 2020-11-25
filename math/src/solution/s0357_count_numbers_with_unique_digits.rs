#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(1)
    pub fn count_numbers_with_unique_digits(mut n: i32) -> i32 {
      if n == 0 {
        return 1;
      }

      let mut res = 10;
      let mut uniqe_digit = 9;
      let mut avail_num = 9;

      
      while n > 1 && avail_num > 0 {
        uniqe_digit = uniqe_digit * avail_num;
        res += uniqe_digit;
        avail_num -= 1;
        n -= 1;
      }

      res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_357() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    }
}
