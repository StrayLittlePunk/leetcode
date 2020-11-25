#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n^2) O(1)
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }
        // Container to keep the size of the largest divisible subset
        //   that ends with each of the nums.
        let mut dp = vec![0; nums.len()];

        // sort the original list in ascending order
        nums.sort();

        let mut max_subset_size = -1;
        let mut max_subset_idx: i32 = -1;

        for i in 0..nums.len() {
            let mut subset_size = 0;

            // find the size of the largest divisible subset
            for k in 0..i {
                if nums[i] % nums[k] == 0 && subset_size < dp[k] {
                    subset_size = dp[k];
                }
            }

            // extend the found subset with the element itself
            dp[i] = subset_size + 1;

            // We reuse this loop to obtain the largest subset size
            //   in order to prepare for the reconstruction of subset.
            if max_subset_size < dp[i] {
                max_subset_size = dp[i];
                max_subset_idx = i as i32;
            }
        }

        /* Reconstruct the largest divisible subset  */
        let mut ans = vec![];
        let mut cur_size = max_subset_size;
        let mut cur_tail = nums[max_subset_idx as usize];
        for i in (0..=max_subset_idx as usize).rev() {
            if cur_size == 0 {
                break;
            }

            if cur_tail % nums[i] == 0 && cur_size == dp[i] {
                ans.push(nums[i]);
                cur_tail = nums[i];
                cur_size -= 1;
            }
        }

        ans.reverse();

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 3]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
            vec![1, 2, 4, 8]
        );
    }
}
