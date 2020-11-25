#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    // O(n^3) O(n)
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = HashSet::new();
        nums.sort();
        if nums.len() < 4 || nums[0] * 4 > target || target > nums[nums.len() - 1] * 4 {
            return Vec::from_iter(ans);
        }

        for i in 0..nums.len() {
            for k in i + 1..nums.len() {
                let (mut lptr, mut rptr) = (k + 1, nums.len() - 1);
                while lptr < rptr {
                    let sum = nums[i] + nums[k] + nums[lptr] + nums[rptr];
                    if sum > target {
                        rptr -= 1;
                    } else if sum < target {
                        lptr += 1;
                    } else {
                        ans.insert(vec![nums[i], nums[k], nums[lptr], nums[rptr]]);
                        lptr += 1;
                        rptr -= 1;
                    }
                }
            }
        }

        Vec::from_iter(ans)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_18() {
        // assert_eq!(
            // Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            // vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        // );
        assert_eq!(Solution::four_sum(vec![], 0), Vec::<Vec<i32>>::new());
    }
}
