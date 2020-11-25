#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
impl Solution {
    // O(log10(n)) O(1)
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;

        let mut i: i64 = 1;
        while i <= n as i64{
            let divider = (i * 10) as i64;

            count += (n as i64 / divider  * i
                + min(max(n as i64 % divider - i + 1, 0), i));

            i *= 10;
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_233() {
        assert_eq!(Solution::count_digit_one(13), 6);
    }
}
