#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(1)
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut zero_count = 0;
        let mut i = 5;
        while i < n + 1 {
            let mut cur = i;
            while cur % 5 == 0 {
                zero_count += 1;
                cur /= 5;
            }

            i += 5;
        }

        zero_count
    }

    // O(log n) O(1)
    pub fn trailing_zeroes_efficent(mut n: i32) -> i32 {
      let mut zero_count = 0;
      while n > 0 {
        n /= 5;
        zero_count += n;
      }


      zero_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_172() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
