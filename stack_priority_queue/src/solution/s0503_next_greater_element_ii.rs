#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut s = vec![];
        let mut res = vec![-1; nums.len()];
        for k in 0..(nums.len() * 2) {
            let i = k % nums.len();
            while !s.is_empty() && nums[s[s.len() - 1]] < nums[i] {
                let j = s[s.len() - 1];
                s.pop();
                res[j] = nums[i];
            }
            s.push(i);
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_503() {}
}
