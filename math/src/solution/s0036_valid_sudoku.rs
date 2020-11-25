#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
impl Solution {
  // O(mn) O(mn)
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let (mut rows, mut cols, mut boxes) = (
            vec![HashSet::<char>::new(); 9],
            vec![HashSet::<char>::new(); 9],
            vec![HashSet::<char>::new(); 9],
        );

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                let ch = board[row][col];
                if ch == '.' {
                    continue;
                }

                if rows[row].contains(&ch) {
                    return false;
                } else {
                    rows[row].insert(ch);
                }

                if cols[col].contains(&ch) {
                    return false;
                } else {
                    cols[col].insert(ch);
                }

                if boxes[(row / 3) * 3 + col / 3].contains(&ch) {
                    return false;
                } else {
                    boxes[(row / 3) * 3 + col / 3].insert(ch);
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
    fn test_1() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}
