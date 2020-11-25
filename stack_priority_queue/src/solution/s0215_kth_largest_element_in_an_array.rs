#![allow(unused)]
pub struct Solution {}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    // Time O(Nlog k), Space O(k) use heap
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
      if k > nums.len() as i32{
        return 0;
      }
      let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k as usize);
      for num in nums {
        heap.push(Reverse(num));
        if heap.len() > k as usize {
          heap.pop();
        }

      }

      match heap.pop().unwrap() {
        Reverse(i) => i,
        _ => 0,
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_215() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2),
            5
        );
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
