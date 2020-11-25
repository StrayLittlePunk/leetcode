#![allow(unused)]
pub struct Solution {}

// amazon interview
impl Solution {
    fn backtrack(n: usize, nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, start: usize) {
        if start == n {
            ans.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            Self::swap(nums, start, i);
            Self::backtrack(n, nums, ans, start + 1);
            Self::swap(nums, start, i);
        }
    }

    fn swap(nums: &mut Vec<i32>, start: usize, end: usize) {
        let t = nums[start];
        nums[start] = nums[end];
        nums[end] = t;
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let n = nums.len();
        Self::backtrack(n, &mut nums, &mut ans, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46() {
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 2, 1],
                vec![3, 1, 2],
            ]
        );
    }
}
