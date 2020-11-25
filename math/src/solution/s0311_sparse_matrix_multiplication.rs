#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(M * N * H) M is A's row, N is A's column, H is B's column O(1)
    pub fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (ma, na, nb) = (a.len(), a[0].len(), b[0].len());
        let mut ans = vec![vec![0; nb]; ma];

        for i in 0..ma {
            for k in 0..na {
                if a[i][k] != 0 {
                    for j in 0..nb {
                        if b[k][j] != 0 {
                            ans[i][j] += a[i][k] * b[k][j];
                        }
                    }
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_311() {
        assert_eq!(
            Solution::multiply(
                vec![vec![1, 0, 0], vec![-1, 0, 3]],
                vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]
            ),
            vec![vec![7, 0, 0], vec![-7, 0, 3]]
        );
    }
}
