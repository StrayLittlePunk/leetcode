#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_owned();
        }

        let mut ans = String::from("");

        // if either one is negative
        if (numerator < 0) ^ (denominator < 0) {
            ans.push('-');
        }

        // convert to long or else abs(-2147483648) overflows
        let (numerator, denominator) = (
            i64::abs(numerator as i64),
            i64::abs(denominator as i64),
        );
        ans.push_str((numerator / denominator).to_string().as_str());
        let mut remainder = numerator % denominator;
        if remainder == 0 {
            return ans;
        }

        ans.push('.');
        let mut map = HashMap::new();
        while remainder != 0 {
            if map.contains_key(&remainder) {
                ans.insert(*map.get(&remainder).unwrap(), '(');
                ans.push(')');
                break;
            }

            map.insert(remainder, ans.len());
            remainder *= 10;
            ans.push(std::char::from_digit((remainder / denominator) as u32, 10).unwrap());
            remainder %= denominator;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_166() {
        assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5".to_owned());
        assert_eq!(Solution::fraction_to_decimal(-50, 8), "-6.25".to_owned());
        assert_eq!(Solution::fraction_to_decimal(2, 1), "2".to_owned());
        assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)".to_owned());
        assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)".to_owned());
        assert_eq!(Solution::fraction_to_decimal(1, 5), "0.2".to_owned());
    }
}
