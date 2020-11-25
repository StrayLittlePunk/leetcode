#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
      if n == 1{
        return 1;
      }
      let (mut first, mut second) = (1, 2);
      for i in 3..=n {
        let tmp = first  + second;
        first = second;
        second = tmp;
      }

      second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_70() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
