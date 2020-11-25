#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
  // O(n) O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut map = HashMap::new();
        for k in 0..nums.len() {
            if !map.contains_key(&nums[k]) {
                map.insert(target - nums[k], k);
            } else {
                ans.push(*map.get(&nums[k]).unwrap() as i32);
                ans.push(k as i32);
                return ans;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
    }
}
