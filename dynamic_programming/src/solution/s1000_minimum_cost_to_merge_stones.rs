#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if (stones.len() - 1) % (k - 1) != 0 {
            return -1;
        }
        let mut dp = vec![vec![0; stones.len()]; stones.len()];
        let sum = {
            let mut sum = Vec::with_capacity(stones.len() + 1);
            sum.push(0);
            let mut cur = 0;
            sum.extend(stones.iter().map(|n| {
                cur = cur + n;
                cur.clone()
            }));
            sum
        };
        for len in (k - 1)..stones.len() {
            for i in 0..(stones.len() - len) {
                let j = i + len;
                dp[i][j] = (i..j)
                    .step_by(k - 1)
                    .map(|l| dp[i][l] + dp[l + 1][j])
                    .min()
                    .unwrap();
                if (j - i) % (k - 1) == 0 {
                    dp[i][j] += sum[j + 1] - sum[i];
                }
            }
        }
        dp[0][stones.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1000() {}
}
