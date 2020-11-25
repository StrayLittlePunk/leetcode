#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n) O(1)
      pub fn integer_break(mut n: i32) -> i32 {
        if n == 2 {
          return 1;
        }
        if n == 3 {
          return 2;
        }

        let mut product = 1;

        while n > 4 {
          product *= 3;
          n -= 3;
        }

        product * n

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_343() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
