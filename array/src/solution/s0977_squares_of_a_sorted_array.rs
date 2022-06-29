#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        let mut rs = vec![0; nums.len()];
        let (mut i, mut j, mut p) = (0, nums.len() - 1, nums.len() - 1);
        loop {
            if nums[i] + nums[j] <= 0 {
                rs[p] = nums[i] * nums[i];
                i += 1;
            } else {
                rs[p] = nums[j] * nums[j];
                if j == 0 {
                    break;
                } // avoid overflow
                j -= 1;
            }
            if p == 0 {
                break;
            }
            p -= 1; // avoid overflow
        }
        rs
    }
}

#[test]
fn testcase() {
    assert_eq!(
        Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
        [0, 1, 9, 16, 100]
    );
    assert_eq!(
        Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
        [4, 9, 9, 49, 121]
    );
}
