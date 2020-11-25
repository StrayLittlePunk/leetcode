#![allow(unused)]
pub struct Solution {}

const DIRECTIONS: [(i32, i32, usize); 4] = [(0, 1, 0), (1, 0, 0), (0, -1, 1), (-1, 0, 1)];

impl Solution {
    // O(max(R, C)^2) O(1)
    pub fn spiral_matrix_iii(r: i32, c: i32, mut r0: i32, mut c0: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let mut t = 0;
        ans.push(vec![r0, c0]);
        if r * c == 1 {
            return ans;
        }

        let mut k = 1;
        while k < 2 * (r + c) as usize {
            for di in DIRECTIONS.iter() {
                // number of steps in this direction;
                let dk = di.2 + k;
                // for each step in this direction
                for j in 0..dk {
                    // step in the i-th direction
                    r0 += di.0;
                    c0 += di.1;
                    if r0 >= 0 && r0 < r && c0 >= 0 && c0 < c {
                        ans.push(vec![r0, c0]);
                        t += 1;
                        if t == r * c {
                            return ans;
                        }
                    }
                }
            }

            k += 2;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_885() {
        assert_eq!(
            Solution::spiral_matrix_iii(1, 4, 0, 0),
            vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![0, 3]]
        );
        assert_eq!(
            Solution::spiral_matrix_iii(5, 6, 1, 4),
            vec![
                vec![1, 4],
                vec![1, 5],
                vec![2, 5],
                vec![2, 4],
                vec![2, 3],
                vec![1, 3],
                vec![0, 3],
                vec![0, 4],
                vec![0, 5],
                vec![3, 5],
                vec![3, 4],
                vec![3, 3],
                vec![3, 2],
                vec![2, 2],
                vec![1, 2],
                vec![0, 2],
                vec![4, 5],
                vec![4, 4],
                vec![4, 3],
                vec![4, 2],
                vec![4, 1],
                vec![3, 1],
                vec![2, 1],
                vec![1, 1],
                vec![0, 1],
                vec![4, 0],
                vec![3, 0],
                vec![2, 0],
                vec![1, 0],
                vec![0, 0]
            ]
        );
    }
}
