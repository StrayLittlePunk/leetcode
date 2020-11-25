#![allow(unused)]

struct NumMatrix {
    dp: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return Self { dp: vec![] };
        }

        let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                dp[r + 1][c + 1] = dp[r + 1][c] + dp[r][c + 1] + matrix[r][c] - dp[r][c];
            }
        }

        Self { dp }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.dp[row2 as usize + 1][col2 as usize + 1]
            - self.dp[row1 as usize][col2 as usize + 1]
            - self.dp[row2 as usize + 1][col1 as usize]
            + self.dp[row1 as usize][col1 as usize]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
