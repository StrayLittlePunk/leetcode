#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(candidates: &Vec<i32>, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, start: usize, target: i32) {
        if target == 0 {
            ans.push(cur.clone());
            return;
        } else if target < 0 {
            return;
        }

        for i in start..candidates.len() {
          // skip dups
          if i > start && candidates[i] == candidates[i-1] {
            continue;
          }
            cur.push(candidates[i]);
            Self::backtrack(candidates, ans, cur, i+1, target - candidates[i]);
            cur.pop();
        }
    }

    // O(N^(T/M+ 1)) O(T/M) 
    // Let N be the number of candidates, T be the target value, and M be the minimal value 
    // among the candidates.
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        candidates.sort();
        Self::backtrack(&candidates, &mut ans, &mut Vec::new(), 0, target);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_40() {
        // assert_eq!(
            // Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8),
            // vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        // );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
