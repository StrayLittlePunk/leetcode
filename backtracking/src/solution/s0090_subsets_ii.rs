#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n * 2^n) O(n)
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        nums.sort();

        let mut temp = vec![];
        Self::backtrack(&nums, 0, &mut temp, &mut ans);

        ans
    }

    fn backtrack(nums: &Vec<i32>, start: usize, temp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        ans.push(temp.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            temp.push(nums[i]);
            Self::backtrack(nums, i + 1, temp, ans);
            temp.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_90() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );
    }
}
