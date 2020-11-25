#![allow(unused)]

struct TicTacToe {
    rows: Vec<i32>,
    cols: Vec<i32>,
    diagonal: i32,
    antdiagonal: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TicTacToe {
    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        Self {
            rows: vec![0; n as usize],
            cols: vec![0; n as usize],
            diagonal: 0,
            antdiagonal: 0,
        }
    }

    /** Player {player} makes a move at ({row}, {col}).
    @param row The row of the board.
    @param col The column of the board.
    @param player The player, can be either 1 or 2.
    @return The current winning condition, can be either:
            0: No one wins.
            1: Player 1 wins.
            2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let (row, col) = (row as usize, col as usize);
        let to_add = if player == 1 { 1 } else { -1 };

        self.rows[row] += to_add;
        self.cols[col] += to_add;

        if row == col {
            self.diagonal += to_add;
        }

        if col == (self.cols.len() - row - 1) {
            self.antdiagonal += to_add;
        }

        let size = self.rows.len();
        if i32::abs(self.rows[row]) == size as i32
            || i32::abs(self.cols[col]) == size as i32
            || i32::abs(self.diagonal) == size as i32
            || i32::abs(self.antdiagonal) == size as i32
        {
            return player;
        }

        return 0;
    }
}

/**
 * Your TicTacToe object will be instantiated and called as such:
 * let obj = TicTacToe::new(n);
 * let ret_1: i32 = obj.make_a_move(row, col, player);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_346() {}
}
