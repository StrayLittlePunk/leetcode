#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        println!();
        let (mut start, mut end) = (0, nums.len());
        while start < end {
            let mid = start + (end - start) / 2;
            if nums[mid] > target {
                end = mid;
            } else if nums[mid] < target {
                start = mid + 1;
            } else {
                return mid as i32;
            }
        }
        return -1;
    }
}

#[test]
fn testcase() {
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
}
