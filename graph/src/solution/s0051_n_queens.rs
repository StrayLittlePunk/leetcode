#![allow(unused)]
pub struct Solution {}


impl Solution {
    // Time O(N!) Space O(N)
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // 路径：board 中小于 row 的那些行都已经成功放置了皇后
        // 选择列表：第 row 行的所有列都是放置皇后的选择
        // 结束条件：row 超过 board 的最后一行
        fn backtrack(board: &mut Vec<Vec<char>>, row: i32, ans: &mut Vec<Vec<String>>) {
            // 触发结束条件
            if row as usize == board.len() {
                let mut solution = vec![];
                for r in board.iter() {
                    solution.push(r.iter().collect());
                }
                ans.push(solution);
                return;
            }

            let n = board[row as usize].len();
            for col in 0..n {
                // 排除不合法选择
                if !is_valid(board, row, col as i32) {
                    continue;
                }

                // 做选择
                board[row as usize][col as usize] = 'Q';
                // 进入下一行决策
                backtrack(board, row + 1, ans);

                // 撤销选择
                board[row as usize][col as usize] = '.';
            }
        }

        /* 是否可以在 board[row][col] 放置皇后？ */
        fn is_valid(board: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
            let n = board.len();

            // 检查列是否有皇后互相冲突
            for i in 0..(row + 1) as usize {
                if board[i][col as usize] == 'Q' {
                    return false;
                }
            }

            // 检查右上方是否有皇后互相冲突
            let mut i = row - 1;
            let mut j = col + 1;
            while i >= 0 && j < n as i32 {
                if board[i as usize][j as usize] == 'Q' {
                    return false;
                }
                i -= 1;
                j += 1;
            }

            // 检查左上方是否有皇后互相冲突
            let mut i = row - 1;
            let mut j = col - 1;
            while i >= 0 && j >= 0 {
                if board[i as usize][j as usize] == 'Q' {
                    return false;
                }
                i -= 1;
                j -= 1;
            }

            true
        }

        let mut ans = vec![];
        let mut board = vec![vec!['.'; n as usize]; n as usize];
        backtrack(&mut board, 0, &mut ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_51() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    ".Q..".to_string(), // Solution 1
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string(),
                ],
                vec![
                    "..Q.".to_string(), // Solution 2
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string(),
                ]
            ]
        );
    }
}
