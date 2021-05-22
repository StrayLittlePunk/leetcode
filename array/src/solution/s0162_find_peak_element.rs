#![allow(unused)]

pub struct Solution {}

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut start, mut end) = (0, nums.len() - 1);

        while start + 1 < end {
            let mid = start + (end - start) / 2;
            if nums[mid] < nums[mid + 1] {
                start = mid;
            } else if nums[mid] > nums[mid + 1] {
                end = mid;
            }
        }

        if nums[start] > nums[end] {
            start as i32
        } else {
            end as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_162() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 5);
    }
}
