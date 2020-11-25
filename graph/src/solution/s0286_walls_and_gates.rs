#![allow(unused)]
pub struct Solution {}

use std::collections::VecDeque;

const EMPTY: i32 = std::i32::MAX;
const GATE: i32 = 0;
const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Solution {
  // Time O(n * m) Space O(m * n)
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
      if rooms.len() < 1 {
        return;
      }

      let m = rooms.len();
      let n = rooms[0].len();

      let mut queue = VecDeque::new();
      for row in 0..m {
        for col in 0..n {
          if rooms[row][col] == GATE {
            queue.push_back((row as i32, col as i32));
          }
        }
      }

      while let Some((row, col)) = queue.pop_front() {
        for direction in DIRECTIONS.iter() {
          let r = row + direction.0;
          let c = col + direction.1;
          if r < 0 || c < 0 || r >= m as i32 || c >= n as i32 || rooms[r as usize][c as usize] != EMPTY {
            continue;
          }

          rooms[r as usize][c as usize] = rooms[row as usize][col as usize] + 1;
          queue.push_back((r, c));
        }
      }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_286() {
        let mut rooms = vec![
            vec![EMPTY, -1, 0, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, -1],
            vec![EMPTY, -1, EMPTY, -1],
            vec![0, -1, EMPTY, EMPTY],
        ];
        Solution::walls_and_gates(&mut rooms);

        assert_eq!(
            rooms,
            vec![
                vec![3, -1, 0, 1],
                vec![2, 2, 1, -1],
                vec![1, -1, 2, -1],
                vec![0, -1, 3, 4]
            ]
        );
    }
}
