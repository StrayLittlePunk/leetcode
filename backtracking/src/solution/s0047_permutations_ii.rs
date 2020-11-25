#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(
        n: usize,
        nums: &Vec<i32>,
        cur: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        start: usize,
        vis: &mut Vec<bool>,
    ) {
        if start == n {
            ans.push(cur.clone());
            return;
        }

        for i in 0..nums.len() {
            if vis[i] || (i > 0 && nums[i] == nums[i - 1] && !vis[i - 1]) {
                continue;
            }

            cur.push(nums[i]);
            vis[i] = true;
            Self::backtrack(n, nums, cur, ans, start + 1, vis);
            vis[i] = false;
            cur.pop();
        }
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let n = nums.len();
        let mut vis = vec![false; n];
        nums.sort();
        Self::backtrack(n, &nums, &mut Vec::new(), &mut ans, 0, &mut vis);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_47() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2,]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ]
        );
        assert_eq!(
            Solution::permute_unique(vec![3, 3, 0, 3]),
            vec![
                vec![0, 3, 3, 3],
                vec![3, 0, 3, 3],
                vec![3, 3, 0, 3],
                vec![3, 3, 3, 0]
            ]
        );
    }
}
