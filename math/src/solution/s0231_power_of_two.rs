#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
       if n == 0 {
         return false;
       } 

       let x = n as i64;
       
       (x & -x) == x 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_231() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
        assert_eq!(Solution::is_power_of_two(4), true);
        assert_eq!(Solution::is_power_of_two(5), false);
    }
}
