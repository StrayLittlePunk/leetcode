#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(1) O(1)
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut rev_x = 0;
        while x > rev_x {
            rev_x = rev_x * 10 + x % 10;
            x /= 10;
        }

        // When the length is an odd number, we can get rid of the middle digit by revertedNumber/10
        // For example when the input is 12321, at the end of the while loop we get x = 12, 
        // revertedNumber = 123,
        // since the middle digit doesn't matter in palidrome(it will always equal to itself), 
        // we can simply get rid of it.
        rev_x == x || x == rev_x / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_125() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
