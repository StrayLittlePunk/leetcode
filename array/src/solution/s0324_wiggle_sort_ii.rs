#![allow(unused)]
pub struct Solution {}

use std::cmp;
use std::i32;
use std::collections::HashMap;

// reference https://leetcode.com/problems/wiggle-sort-ii/discuss/628101/Rust-O(n)-time-and-O(n)-time-solution-beats-100-if-the-solutions.
pub fn find_kth_largest(nums: &Vec<i32>, mut k: i32) -> i32 {
    let mut map = HashMap::new();
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for num in nums {
        let counter = map.entry(num).or_insert(0);
        *counter += 1;
        min = cmp::min(min, *num);
        max = cmp::max(max, *num);
    }
    for i in (min..=max).rev() {
        if let Some(x) = map.get(&i) {
            if k - x <= 0 {
                return i;
            }
            k = k - x;
        }
    }
    unreachable!();
}

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        let n = nums.len();
        let mut k = n - 1;

        pub fn index(i: usize, n: usize) -> usize {
            (1 + 2 * i) % (n | 1)
        }

        let mid = find_kth_largest(nums, (n / 2) as i32);

        while j <= k {
            match (nums[index(j, n)]).cmp(&mid) {
                cmp::Ordering::Less => {
                    nums.swap(index(j, n), index(k, n));
                    k -= 1;
                }
                cmp::Ordering::Greater => {
                    nums.swap(index(j, n), index(i, n));
                    j += 1;
                    i += 1;
                }
                cmp::Ordering::Equal => j += 1,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_324() {
        let mut v = vec![1, 5, 1, 1, 6, 4];
        Solution::wiggle_sort(&mut v);
        assert_eq!(v, vec![1, 5, 1, 6, 1, 4]);
        let mut v = vec![1, 3, 2, 2, 3, 1];
        Solution::wiggle_sort(&mut v);
        assert_eq!(v, vec![2, 3, 1, 3, 1, 2]);
    }
}
