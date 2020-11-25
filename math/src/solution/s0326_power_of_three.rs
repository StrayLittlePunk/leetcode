#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(log n) O(1)
      pub fn is_power_of_three(mut n: i32) -> bool {
        if n < 1{
          return false;
        }

        while n % 3 == 0 {
          n /= 3;
        }

        n == 1

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_326() {
        assert_eq!(Solution::is_power_of_three(27), true);
        assert_eq!(Solution::is_power_of_three(0), false);
        assert_eq!(Solution::is_power_of_three(9), true);
        assert_eq!(Solution::is_power_of_three(45), false);
    }
}
