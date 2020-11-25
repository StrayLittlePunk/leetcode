#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        // base case
        if costs.len() == 0 {
            return 0;
        }

        let mut dp = vec![];
        for &cost in costs[0].iter() {
            dp.push(cost);
        }

        for i in 1..costs.len() {
            let min_cost0 = min(dp[1], dp[2]) + costs[i][0];
            let min_cost1 = min(dp[0], dp[2]) + costs[i][1];
            let min_cost2 = min(dp[0], dp[1]) + costs[i][2];
            dp[0] = min_cost0;
            dp[1] = min_cost1;
            dp[2] = min_cost2;
        }

        *dp.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_256() {
        assert_eq!(
            Solution::min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]]),
            10
        );
        assert_eq!(Solution::min_cost(vec![]), 0);
        assert_eq!(Solution::min_cost(vec![vec![7, 6, 2]]), 2);
    }
}
