#![allow(unused)]
pub struct Solution {}

use std::i32::MAX;

impl Solution {
  // O(n^2) O(1)
    pub fn three_sum_closest(mut nums: Vec<i32>, mut target: i32) -> i32 {

        nums.sort();

        let mut distance = MAX;

        for i in 0..nums.len() {
            if distance == 0 {
                break;
            }

            let (mut lptr, mut rptr) = (i + 1, nums.len() - 1);
            while lptr < rptr {
                let sum = nums[i] + nums[lptr] + nums[rptr];
                if i32::abs(target - sum) < i32::abs(distance) {
                    distance = target - sum;
                }

                if sum < target {
                    lptr += 1;
                } else {
                    rptr -= 1;
                }
            }
        }

        target - distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![-4, -1, 1, 2], 1), 2);
    }
}
