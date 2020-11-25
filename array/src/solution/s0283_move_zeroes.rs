#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {

      let mut last_nonzero_foundat = 0;
      for i in 0..nums.len() {
        if nums[i] != 0 {
          nums.swap(last_nonzero_foundat, i);
          last_nonzero_foundat +=1;
        }
      }

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
