#![allow(unused)]

pub struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut l: i32 = 0;
        let mut mid: i32 = 0;
        let mut r: i32 =  (nums.len() - 1) as i32;

        while r > l {
          mid = l + (r - l) / 2;
          if nums[mid as usize] > nums[(mid +1) as usize] {
            r = mid;
          } else {
            l = mid + 1;
          }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
