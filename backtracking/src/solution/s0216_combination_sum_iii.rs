#![allow(unused)]
pub struct Solution {}

impl Solution {
    fn backtrack(k: usize, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, start: usize, target: i32) {
        if target == 0 && k == cur.len() {
            ans.push(cur.clone());
        } else if target < 0 || k < cur.len() {
            return;
        }

        for i in start..=9 {
            cur.push(i as i32);
            Self::backtrack(k, ans, cur, i + 1, target - i as i32);
            cur.pop();
        }
    }

    // O(9!K/(9-K)!) O(K)
    // Let K be the number of digits in a combination.
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::backtrack(k as usize, &mut ans, &mut Vec::new(), 1, n);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_216() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4],]);
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
        assert_eq!(Solution::combination_sum3(4, 1), Vec::<Vec<i32>>::new());
    }
}
