#![allow(unused)]
pub struct Solution {}
// apple interview
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let (mut sum, mut leftsum) = (0, 0);
        for i in 0..nums.len() {
            sum += nums[i];
        }
        for i in 0..nums.len() {
            if leftsum == sum - leftsum - nums[i] {
                return i as i32;
            }
            leftsum += nums[i];
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_724() {}
}
