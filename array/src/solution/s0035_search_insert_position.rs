#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
       let mut left: i32 = 0;
       let mut right: i32 = (nums.len() - 1) as i32;
       let mut ans: i32 = nums.len() as i32;

       while left <= right {
         let mid = ((right - left) >> 1) + left;
         if target <= nums[mid as usize] {
           ans = mid;
           right = mid - 1;
         } else {
           left = mid + 1;
         }
       }
       ans 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_35() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
}
