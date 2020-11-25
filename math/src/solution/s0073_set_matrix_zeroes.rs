#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(M X N) O(1)
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rlen = matrix.len();
        let clen = matrix[0].len();
        let mut is_col = false;

        for i in 0..rlen {
            // Since first cell for both first row and first column is the same i.e. matrix[0][0]
            // We can use an additional variable for either the first row/column.
            // For this solution we are using an additional variable for the first column
            // and using matrix[0][0] for the first row.
            if matrix[i][0] == 0 {
                is_col = true;
            }
            for j in 1..clen {
                // If an element is zero, we set the first element of the corresponding row and 
                // column to 0
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        // Iterate over the array once again and using the first row and first column, update 
        // the elements.
        for i in 1..rlen {
            for j in 1..clen {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // See if the first row needs to be set to zero as well
        if matrix[0][0] == 0 {
            for j in 0..clen {
                matrix[0][j] = 0;
            }
        }

        // See if the first column needs to be set to zero as well
        if is_col {
            for i in 0..rlen {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_73() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
