#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(nk) O(1)
    pub fn min_cost_ii(mut costs: Vec<Vec<i32>>) -> i32 {
        // base case
        if costs.len() == 0 {
            return 0;
        }

        for i in 1..costs.len() {
            let mut min_color = costs[i].len() + 1;
            let mut secmin_color = costs[i].len() + 1;
            for j in 0..costs[i].len() {
                let cost = costs[i - 1][j];
                if min_color == costs[i].len() + 1 || cost < costs[i - 1][min_color] {
                    secmin_color = min_color;
                    min_color = j;
                } else if secmin_color == costs[i].len() + 1 || cost < costs[i - 1][secmin_color] {
                    secmin_color = j;
                }
            }

            for j in 0..costs[i].len() {
                if j == min_color {
                    costs[i][j] += costs[i - 1][secmin_color];
                } else {
                    costs[i][j] += costs[i - 1][min_color];
                }
            }
        }

        *costs[costs.len() - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_265() {
        assert_eq!(Solution::min_cost_ii(vec![vec![1, 5, 3], vec![2, 9, 4]]), 5);
    }
}
