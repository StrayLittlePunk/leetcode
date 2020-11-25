#![allow(unused)]

pub struct Solution {}

// https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/
// reference: https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/discuss/77784/O(n)-super-clean-9-line-Java-solution-with-HashMap

use std::cmp::max;
use std::collections::HashMap;
impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        let mut max_v = 0;

        for i in 0..nums.len() {
          sum += nums[i];
          if sum == k {
            max_v = i + 1;
          } else if hm.contains_key(&(sum - k)) {
            max_v = max(max_v, i - *hm.get(&(sum - k)).expect("Not found") as usize);
          }
          if !hm.contains_key(&sum) {
            hm.insert(sum, i as i32);
          }
        }
        max_v as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_325() {
        assert_eq!(Solution::max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
        assert_eq!(Solution::max_sub_array_len(vec![-2, -1, 2, 1], 1), 2);
    }
}
