#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(start: i32, n: i32, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
        if n <= 1 && cur.len() > 1 {
            ans.push(cur.clone());
            return;
        }

        for i in start..=n {
            if n % i != 0 {
                continue;
            }
            cur.push(i);
            Self::backtrack(i, n / i, ans, cur);
            cur.pop();
        }
    }

    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::backtrack(2, n, &mut ans, &mut Vec::new());

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_254() {
        assert_eq!(Solution::get_factors(1), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::get_factors(37), Vec::<Vec<i32>>::new());
        assert_eq!(
            Solution::get_factors(12),
            vec![vec![2, 2, 3], vec![2, 6], vec![3, 4]]
        );
        assert_eq!(
            Solution::get_factors(32),
            vec![
                vec![2, 2, 2, 2, 2],
                vec![2, 2, 2, 4],
                vec![2, 2, 8],
                vec![2, 4, 4],
                vec![2, 16],
                vec![4, 8]
            ]
        );
    }
}
