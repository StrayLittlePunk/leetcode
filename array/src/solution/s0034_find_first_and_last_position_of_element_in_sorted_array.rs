#![allow(unused)]

pub struct Solution {}

impl Solution {
    fn extreme_insertion_index(nums: &Vec<i32>, target: i32, flag: bool) -> i32 {
        let mut l: i32 = 0;
        let mut mid: i32 = 0;
        let mut r: i32 =  nums.len() as i32;

        while l < r {
          mid = (l + r) /2;
          if nums[mid as usize] > target || (flag && target == nums[mid as usize]) {
            r = mid;
          } else {
            l = mid + 1;
          }
        }
      l
    }
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let mut res: Vec<i32> = vec![-1, -1];

        let left_idx = Solution::extreme_insertion_index(&nums, target, true);

        if left_idx == nums.len() as i32 || nums[left_idx as usize] != target {
          return res;
        }

        res[0] = left_idx;
        res[1] = Solution::extreme_insertion_index(&nums, target, false) - 1;

       res 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }
}
