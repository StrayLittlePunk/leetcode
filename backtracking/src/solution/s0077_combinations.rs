#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
      let mut ans = vec![];
      Self::backtrack(n as usize, &mut ans, &mut Vec::new(), 1, k as usize);

      return ans;
    }

    fn backtrack(n: usize, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, idx: usize, k: usize) {
      if cur.len() == k {
        ans.push(cur.clone());
      }

      for i in idx..=n {
        cur.push(i as i32);
        Self::backtrack(n, ans, cur, i + 1, k);
        cur.pop();
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_77() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
    }
}
