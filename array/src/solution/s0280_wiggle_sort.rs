#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>){
      if nums.is_empty() {
        return;
      }
      for i in 0..(nums.len() - 1) {
        if ((i % 2 == 0) && (nums[i] > nums[i + 1])) ||
          ((i % 2 == 1) && (nums[i] < nums[i + 1])) {
            nums.swap(i, i+1);
        }
      }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_280() {
        let mut v = vec![3, 5, 2, 1, 6, 4];
        Solution::wiggle_sort(&mut v);
        assert_eq!(v, vec![3, 5, 1, 6, 2, 4]);
    }
}
