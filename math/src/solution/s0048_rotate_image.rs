#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n^2) O(1)
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // transpose matrix
        for i in 0..n {
            for j in i..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }

        // reverse each row
        for i in 0..n {
            matrix[i].reverse();
        }
    }
    // Rotate four rectangles in one single loop
    pub fn rotate_fast(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..(n + 1) / 2 {
            for j in 0..n / 2 {
                let tmp = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = matrix[n - 1 - i][n - j - 1];
                matrix[n - 1 - i][n - j - 1] = matrix[j][n - 1 - i];
                matrix[j][n - 1 - i] = matrix[i][j];
                matrix[i][j] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_48() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

        matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );

        matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);

        matrix = vec![vec![1, 2], vec![3, 4]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
    }
}
