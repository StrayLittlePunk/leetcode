#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32{

      if nums.len() < 2 {
        return nums.len() as i32;
      }

      let mut prediff = nums[1] - nums[0];
      let mut count = if prediff != 0 {2} else {1};

      for i in 2..nums.len() {
        let diff = nums[i] - nums[i -1];
        if (diff >0 && prediff <= 0) || (diff < 0 && prediff >=0) {
          count += 1;
          prediff = diff;
        }
      }

      count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_376() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(
          Solution::wiggle_max_length(
            vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]), 7);
        assert_eq!(
          Solution::wiggle_max_length(
            vec![1, 2, 3, 4, 5, 7, 8, 9]), 2);
    }
}
