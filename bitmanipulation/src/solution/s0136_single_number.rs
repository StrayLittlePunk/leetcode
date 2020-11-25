#![allow(unused)]
pub struct Solution {}

impl Solution {
  // Time O(N), Space O(1) simliar to problem 389,  Analysis see problem 389
    pub fn single_number(nums: Vec<i32>) -> i32 {
      let mut ans = 0;
      for n in nums {
        ans ^= n;
      }

      ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_136() {
        assert_eq!(
            Solution::single_number(vec![2, 2, 1]),
            1
        );
        assert_eq!(
            Solution::single_number(vec![4, 1, 2, 1, 2]),
            4
        );
        assert_eq!(
            Solution::single_number(vec![1]),
            1
        );
    }
}
