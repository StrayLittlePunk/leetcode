#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
      let mut ans: i64 = 0;
      let mut sign = 1;
      
      if x < 0 {
        x = -x;
        sign = -1;
      }
      while x > 0 {
        ans = ans * 10 + x as i64 % 10;
        x /= 10;
      }

      
      ans = ans * sign;
      if ans > i64::pow(2, 31) - 1 || ans < -1 * i64::pow(2, 31) {
        return 0;
      }
      ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
