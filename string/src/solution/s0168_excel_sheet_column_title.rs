#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(log n) O(log n)  log base = 26
    pub fn convert_to_title(mut n: i32) -> String {

        if n == 0 {
            return "".to_string();
        } else {
            n -= 1;
            let mut ret = Self::convert_to_title(n / 26);
            ret.push(std::char::from_u32('A' as u32 + (n as u32 % 26)).unwrap());

            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
    }
}
