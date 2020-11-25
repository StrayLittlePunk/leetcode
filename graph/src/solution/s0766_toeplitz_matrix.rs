#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if row > 0 && col > 0 && matrix[row - 1][col - 1] != matrix[row][col] {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_766() {}
}
