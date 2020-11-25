#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(1), space O(1)
    // https://en.wikipedia.org/wiki/Digital_root#Direct_formulas
    // solve digital root 
    // n != 0, b is base radix ==> 1 + ((n - 1) mod (b - 1))
    // n == 0, ==> 0
    pub fn add_digits(num: i32) -> i32 {
      match num {
        0 => 0,
        _ => 1 +( (num - 1) % (10 - 1)),
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_66() {
        assert_eq!(Solution::add_digits(38), 2);
    }
}
