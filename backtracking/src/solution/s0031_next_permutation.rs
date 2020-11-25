#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn reverse(nums: &mut Vec<i32>, mut start: usize) {
      let mut end = nums.len() - 1;
      while start < end {
        Self::swap(nums, start, end);
        start += 1;
        end -= 1;
      }

    }

    fn swap(nums: &mut Vec<i32>, start: usize, end: usize) {
        let t = nums[start];
        nums[start] = nums[end];
        nums[end] = t;
    }

    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() as i32 - 2;
        while i >= 0 && nums[i as usize + 1] <= nums[i as usize] {
            i -= 1;
        }

        if i >= 0 {
            let mut j = nums.len() as i32 - 1;
            while j >= 0 && nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            Self::swap(nums, i as usize, j as usize);
        }

        Self::reverse(nums, (i + 1) as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
        nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
        nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
        nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
    }
}
