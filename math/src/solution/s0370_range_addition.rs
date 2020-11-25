#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(K +n) O(1)
    // https://leetcode.com/articles/a-recursive-approach-to-segment-trees-range-sum-queries-lazy-propagation/
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        if length < 1 || updates.len() < 1 {
            return vec![0];
        }
        let mut ans = vec![0; length as usize];
        for tuple in updates.iter() {
            let (start, end, val) = (tuple[0], tuple[1], tuple[2]);
            ans[start as usize] += val;
            if end < length - 1 {
                ans[end as usize + 1] -= val;
            }
        }

        for i in 1..ans.len() {
            ans[i] += ans[i - 1];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::get_modified_array(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]]),
            vec![-2, 0, 3, 5, 3]
        );
    }
}
