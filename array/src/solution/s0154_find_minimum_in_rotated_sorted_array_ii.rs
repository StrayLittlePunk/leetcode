#![allow(unused)]

pub struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);
        while start + 1 < end {
            let mid = start + (end - start) / 2;
            if nums[mid] < nums[end] {
                end = mid;
            } else if nums[mid] > nums[end] {
                start = mid;
            } else {
                // when num[mid] and num[end] are same
                end -= 1;
            }
        }

        if nums[start] < nums[end] {
            nums[start]
        } else {
            nums[end]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_153() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    }
}
