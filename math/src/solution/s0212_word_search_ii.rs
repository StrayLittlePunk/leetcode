#![allow(unused)]
pub struct Solution {}

use std::collections::HashSet;
use std::iter::FromIterator;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

impl Solution {
  // backtrack burte-force O(S * N * 3^l) S is the lenght of words vector , N is the 
  // length of board celles , l is the length of words's string
  // O(max(S, l))
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        if board.len() == 0 || board[0].len() == 0 || words.len() == 0 {
            return vec![];
        }

        let mut ans = HashSet::new();
        let (m, n) = (board.len(), board[0].len());
        for word in words.iter() {
            for row in 0..m {
                for col in 0..n {
                    if ans.contains(word) {
                        continue;
                    }

                    if Self::backtrack(
                        row as i32,
                        col as i32,
                        &mut board,
                        m as i32,
                        n as i32,
                        word.as_str(),
                    ) {
                        ans.insert(word.to_string());
                    }
                }
            }
        }

        Vec::from_iter(ans)
    }

    fn backtrack(
        row: i32,
        col: i32,
        board: &mut Vec<Vec<char>>,
        m: i32,
        n: i32,
        word: &str,
    ) -> bool {
        // find a solution
        if word.len() == 0 {
            return true;
        }

        // preare result
        let mut ret = false;

        let ch = word.chars().next().unwrap();
        // check bound
        if row < 0 || col < 0 || row >= m || col >= n || ch != board[row as usize][col as usize] {
            return ret;
        }

        // tag
        board[row as usize][col as usize] = '#';

        for di in DIRECTIONS.iter() {
            let (can_row, can_col) = (row + di.0, col + di.1);
            ret = Self::backtrack(can_row, can_col, board, m, n, &word[1..]);
            if ret {
                break;
            }
        }

        // untag and return false
        board[row as usize][col as usize] = ch;

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_212() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    "oath".to_string(),
                    "pea".to_string(),
                    "eat".to_string(),
                    "rain".to_string(),
                ]
            ),
            vec!["eat".to_string(), "oath".to_string(),]
        );
    }
}
