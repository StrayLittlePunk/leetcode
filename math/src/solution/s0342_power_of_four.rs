#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(log n) O(1)
    pub fn is_power_of_four(num: i32) -> bool {
      num > 0 && ((num & (num - 1)) == 0) && (num % 3 == 1) 
    }
    pub fn is_power_of_four_log(num: i32) -> bool {
      num > 0 && f32::log2(num as f32) % 2.0 == 0.0
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_342() {
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
        assert_eq!(Solution::is_power_of_four_log(16), true);
        assert_eq!(Solution::is_power_of_four_log(5), false);
    }
}
