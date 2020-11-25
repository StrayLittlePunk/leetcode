#![allow(unused)]
pub struct Solution {}

// google interview
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        use std::cmp::max;
        let (mut ans, mut anchor) = (0, 0);
        for i in 0..nums.len() {
            if i > 0 && nums[i - 1] >= nums[i] {
                anchor = i;
            }
            ans = max(ans, i - anchor + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_674() {}
}
