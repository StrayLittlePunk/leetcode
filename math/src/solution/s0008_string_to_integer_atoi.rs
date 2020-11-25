#![allow(unused)]
pub struct Solution {}

use std::i32::{MAX, MIN};
impl Solution {
    // Time O(N), space O(1)
    pub fn my_atoi(s: String) -> i32 {
        // remove whitespace
        let ts = s.trim();
        if ts.len() < 1 {
            return 0;
        }


        // initaial number sign = 1
        let mut sign = 1;
        // initail digit start offset
        let mut start = 0;

        // check negitive
        if &ts[0..1] == "-" {
            sign = -1;
            start = 1;
        } else if &ts[0..1] == "+" {
          start = 1;
        }

        let mut ans = 0;

        for ch in ts[start..].chars() {
            // if not digit ,exit loop
            if !ch.is_ascii_digit() {
                break;
            }

            // parse char to integer
            let n = ch.to_digit(10).unwrap() as i32;

            if (sign == 1 && (MAX - n) / 10 >= ans) || (sign == -1 && (MIN + n) / 10 <= ans * sign) {
                ans = ans * 10 + n;

            // prevent overflow return MAX/MIN
            } else if sign == 1 {
                return MAX;
            } else {
                return MIN;
            }
        }

        ans * sign
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("42   ".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42   ".to_string()), -42);
        assert_eq!(Solution::my_atoi("   4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        //  The number "-91283472332" is out of the range of a 32-bit signed integer.
        //  Thefore INT_MIN (−231) is returned.
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
