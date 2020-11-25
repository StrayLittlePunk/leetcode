#![allow(unused)]
pub struct Solution {}

use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};

impl Solution {
    // O(n^2) O(n)
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        if nums.len() < 3 {
            return ans;
        }

        // Sort the input array nums.
        nums.sort();

        let len = nums.len();

        for i in 0..len {
            // i remain value must bigger than nums[i] because of nums is sorted
            if nums[i] >= 0 {
                break;
            }
            // If the current value is the same as the one before, skip it.
            if i == 0 || nums[i - 1] != nums[i] {
                //Self::two_sum(&nums, i, &mut ans);
                Self::two_sum_pointer(&nums, i, &mut ans);
            }
        }

        ans
    }

    // O(n^2) O(n)
    fn two_sum(nums: &Vec<i32>, i: usize, ans: &mut Vec<Vec<i32>>) {
        let len = nums.len();
        let mut set = HashSet::new();
        let mut k = i + 1;
        while k < len {
            if set.contains(&nums[k]) {
                ans.push(vec![nums[i], -nums[k] - nums[i], nums[k]]);
                // Increment k while the next value is the same
                //  as before to avoid duplicates in the result.
                while k + 1 < len && nums[k] == nums[k + 1] {
                    k += 1;
                }
            } else {
                set.insert(-nums[i] - nums[k]);
            }

            k += 1;
        }
    }


    // O(n^2) O(log n) ~ O(n)
    fn two_sum_pointer(nums: &Vec<i32>, i: usize, ans: &mut Vec<Vec<i32>>) {
        let (mut lptr, mut rptr) = (i + 1, nums.len() - 1);

        while lptr < rptr {
            let s = nums[i] + nums[lptr] + nums[rptr];
            if s > 0 {
                rptr -= 1;
            } else if s < 0 {
                lptr += 1;
            } else {
                ans.push(vec![nums[i], nums[lptr], nums[rptr]]);
                lptr += 1;
                rptr -= 1;
                while lptr < rptr && nums[lptr] == nums[lptr - 1] {
                    lptr += 1;
                }
            }
        }
    }

    // O(n^2) O(n)
    pub fn three_sum_unsort(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = HashSet::new();
        if nums.len() < 3 {
            return Vec::from_iter(ans);
        }

        let len = nums.len();
        let mut map = HashMap::new();
        let mut dups = HashSet::new();
        for i in 0..len {
            if !dups.contains(&nums[i]) {
                dups.insert(nums[i]);

                for k in i + 1..len {
                    let complement = -nums[i] - nums[k];
                    if map.contains_key(&complement) && *map.get(&complement).unwrap() == i {
                        let mut sum = vec![nums[i], nums[k], complement];
                        sum.sort();
                        ans.insert(sum);
                    }

                    map.insert(nums[k], i);
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
    fn test_15() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::three_sum(vec![0]), Vec::<Vec<i32>>::new());
    }
}
