#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(1) O(1)
    pub fn int_to_roman(mut num: i32) -> String {
        let digits = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut ans = "".to_string();

        for &(val, sym) in digits.iter() {
            if num < 0 {
                break;
            }

            while val <= num {
                num -= val;
                ans.push_str(sym);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
