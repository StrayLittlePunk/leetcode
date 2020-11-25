#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(log n) O(1)
      pub fn last_remaining(n: i32) -> i32 {
        let mut left = true;
        let mut remaining = n;
        let mut step = 1;
        let mut head = 1;

        while remaining > 1 {
          if left || remaining % 2 == 1 {
            head += step;
          }

          remaining /= 2;
          step *= 2;
          left = !left;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_390() {
        assert_eq!(Solution::last_remaining(9), 6);
    }
}
