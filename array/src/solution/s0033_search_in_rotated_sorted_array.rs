#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
       if nums.len() == 0 { return -1; }
        let (mut l, mut h) = (0, nums.len());
        while l < h {
            let mid = (l + h) / 2;
            let n = if (nums[mid] < nums[0]) == (target < nums[0]) {
                nums[mid]
            } else {
                if target < nums[0] {
                    std::i32::MIN
                } else {
                    std::i32::MAX
                }
            };
            if n < target {
                l = mid + 1;
            } else if n > target {
                h = mid;
            } else {
                return mid as i32;
            }
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
