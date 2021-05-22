#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start + 1 < end {
            let mid = start + (end - start) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] > nums[start] {
                if target >= nums[start] && target <= nums[mid] {
                    end = mid;
                } else {
                    start = mid;
                }
            } else {
                if target >= nums[mid] && target <= nums[end] {
                    start = mid;
                } else {
                    end = mid;
                }
            }
        }

        if nums[start] == target {
            return start as i32;
        }

        if nums[end] == target {
            return end as i32;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
