#![allow(unused)]
pub struct Solution {}

use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const BFS: bool = true;


impl Solution {
    //Time O(N) Space O(N) N is the number of cells in the board
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() < 1 {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();

        let mut borders = vec![];
        // step1 construct the list of border cells
        for r in 0..rows {
            borders.push((r, 0));
            borders.push((r, cols - 1));
        }

        for c in 0..cols {
            borders.push((0, c));
            borders.push((rows - 1, c));
        }

        // step2 mark the escaped cells
        for (x, y) in borders.iter() {
            if board[*x][*y] != 'O' {
                continue;
            }
            if BFS {
                Self::bfs(board, *x, *y, &rows, &cols);
            } else {
                Self::dfs(board, *x, *y, &rows, &cols);
            }
        }

        // step3 flip the cells to their correct final states
        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                }
                if board[r][c] == 'E' {
                    board[r][c] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, row: usize, col: usize, rows: &usize, cols: &usize) {
        if board[row][col] != 'O' {
            return;
        }

        // mark Visited  in place
        board[row][col] = 'E';
        for direction in DIRECTIONS.iter() {
            let r = (row as i32 + direction.0) as usize;
            let c = (col as i32 + direction.1) as usize;

            if c < *cols - 1 && r < *rows - 1 && c > 0 && r > 0 {
                Self::dfs(board, r, c, rows, cols);
            }
        }
    }

    fn bfs(board: &mut Vec<Vec<char>>, row: usize, col: usize, rows: &usize, cols: &usize) {
        let mut queue = VecDeque::new();

        queue.push_back((row, col));

        while let Some((row, col)) = queue.pop_front() {
            if board[row][col] != 'O' {
                continue;
            }

            board[row][col] = 'E';

            for direction in DIRECTIONS.iter() {
                let r = (row as i32 + direction.0) as usize;
                let c = (col as i32 + direction.1) as usize;
                if c < *cols - 1 && r < *rows - 1 && c > 0 && r > 0 {
                    queue.push_back((r, c));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_130() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);

        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }
}
