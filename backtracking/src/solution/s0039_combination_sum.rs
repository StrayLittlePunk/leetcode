#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(candidates: &Vec<i32>, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, start: usize, target: i32) {
        if target == 0 {
            ans.push(cur.clone());
        } else if target < 0 {
            return;
        }

        for i in start..candidates.len() {
            cur.push(candidates[i]);
            Self::backtrack(candidates, ans, cur, i, target - candidates[i]);
            cur.pop();
        }
    }

    // O(N^(T/M+ 1)) O(T/M) 
    // Let N be the number of candidates, T be the target value, and M be the minimal value 
    // among the candidates.
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::backtrack(&candidates, &mut ans, &mut Vec::new(), 0, target);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7],]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(Solution::combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }
}
