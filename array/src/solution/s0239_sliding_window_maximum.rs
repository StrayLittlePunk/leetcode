#![allow(unused)]
struct Solution{}

// https://leetcode.com/problems/sliding-window-maximum/description/

use std::cmp::max;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
      let n = nums.len();
      if (n as i32) * k == 0 {
        return Vec::new();
      }
      if k == 1 {
        return nums;
      }

      let mut res: Vec<i32> = Vec::new();
      // let mut max_v = nums[0..(k as usize)].iter().max().unwrap();
      
      let mut left = vec![0;n];
      let mut right = vec![0;n];
      left[0] = nums[0];
      right[n -1] = nums[n -1];

      for i in 1..nums.len() {
        if i as i32 % k == 0 {
          left[i] = nums[i]; // block_start
        } else {
          left[i] = max(left[i -1], nums[i]);
        }
        // from right to left
        let j = n - i - 1;
        if (j + 1) as i32 % k == 0 {
          right[j] = nums[j]; // block end
        } else {
          right[j] = max(right[j + 1], nums[j]);
        }
      }

      for i in 0..(n - k as usize + 1) {
        res.push(max(left[i + k as usize - 1], right[i]));
      }
      res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_239() {
        assert_eq!(
          Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), 
          vec![3, 3, 5, 5, 6, 7]);
    }
}
