#![allow(unused)]
pub struct Solution {}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
  // O(N * 3^min(l, N)) N is the number of cells in the board and L
  // is the length of the word to be matched.
  // For the backtracking function, initially we could have at 
  // most 4 directions to explore, but further the choices are reduced into 3 
  // (since we won't go back to where we come from)
  // O(L)
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (m, n) = (board.len(), board[0].len());
        for r in 0..m {
            for c in 0..n {
                if Self::backtrack(
                    r as i32,
                    c as i32,
                    &mut board,
                    m as i32,
                    n as i32,
                    word.as_str(),
                ) {
                    return true;
                }
            }
        }

        false
    }

    fn backtrack(
        row: i32,
        col: i32,
        board: &mut Vec<Vec<char>>,
        m: i32,
        n: i32,
        word: &str,
    ) -> bool {
        /* Step 1). check the bottom case. */
        if word.len() == 0 {
            return true;
        }

        let ori_char = word.chars().next().unwrap();
        /* Step 2). Check the boundaries. */
        if row < 0
            || col < 0
            ||row >= m
            || col >= n
            || board[row as usize][col as usize] != ori_char
        {
            return false;
        }

        /* Step 3). explore the neighbors in DFS */
        // mark the path before the next exploration
        board[row as usize][col as usize] = '#';

        for di in DIRECTIONS.iter() {
            let (can_row, can_col) = (row + di.0, col + di.1);

            if Self::backtrack(can_row, can_col, board, m, n, &word[1..]) {
                // return without cleanup
                return true;
            }
        }

        /* Step 4). clean up and return false. */
        board[row as usize][col as usize] = ori_char;

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {
        assert_eq!(Solution::exist(vec![vec!['A'],], "A".to_string()), true);
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            ),
            false
        );
    }
}
