#![allow(unused)]
pub struct Solution {}
use std::cmp::min;
use std::i32::MAX;
impl Solution {
    pub fn assign_bikes(workers: Vec<Vec<i32>>, bikes: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; 1 << bikes.len()]; workers.len()];

        Self::solve(0, 0, &workers, &bikes, &mut dp)
    }

    fn solve(
        cur: usize,
        takebits: usize,
        workers: &Vec<Vec<i32>>,
        bikes: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if cur == workers.len() {
            return 0;
        } else if dp[cur][takebits] != 0 {
            return dp[cur][takebits];
        }
        let mut best = MAX;
        for i in 0..bikes.len() {
            if (takebits & 1 << i) != 0 {
                continue;
            }
            let mut dist =
                i32::abs(workers[cur][0] - bikes[i][0]) + i32::abs(workers[cur][1] - bikes[i][1]);
            best = min(
                best,
                dist + Self::solve(cur + 1, takebits | (1 << i), workers, bikes, dp),
            );
        }
        dp[cur][takebits] = best;
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1066() {}
}
