#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(n + m) O(1)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
      if matrix.len() == 0 || matrix[0].len() == 0 {
        return false;
      }
      // start out pointer in the bottom-left 
      let (mut row, mut col) = (matrix.len() as i32 -1, 0);

      while row >= 0 && col < matrix[0].len() {
        if matrix[row as usize][col] > target {
          row -= 1;
        } else if matrix[row as usize][col] < target {
          col += 1;
        } else {
          return true;
        }

      }

      false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_240() {
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                5
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30]
                ],
                20
            ),
            false
        );
    }
}
