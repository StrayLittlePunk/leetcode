#![allow(unused)]
pub struct Solution {}

use std::cmp::min;

impl Solution {
    // DP Time O(N) Space O(N) N is input 
    pub fn nth_ugly_number(n: i32) -> i32 {
        if n <= 0 {
            return 1;
        }
        let mut nums: Vec<i32> = Vec::with_capacity(n as usize);
        let (mut i2, mut i3, mut i4, mut i5) = (0, 0, 0, 0);
        nums.push(1);
        for i in 1..n {
            let ugly = min(
                nums[i5] * 5,
                min(nums[i4] * 4, min(nums[i2] * 2, nums[i3] * 3)),
            );
            nums.push(ugly);
            if ugly == nums[i2] * 2 {
              i2 += 1;
            }
            if ugly == nums[i3] * 3 {
              i3 += 1;
            }
            if ugly == nums[i4] * 4 {
              i4 += 1;
            }
            if ugly == nums[i5] * 5 {
              i5 += 1;
            }
        }

        nums[n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_264() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}
