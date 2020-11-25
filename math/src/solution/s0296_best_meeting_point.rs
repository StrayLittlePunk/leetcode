#![allow(unused)]
pub struct Solution {}

impl Solution {
  // O(mnlogmn) O(mn)
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let (mut rows, mut cols) = (vec![], vec![]);

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] == 1 {
                    rows.push(row as i32);
                    cols.push(col as i32);
                }
            }
        }

        // find median
        let row = rows[rows.len() / 2];
        cols.sort();
        let col = cols[cols.len() / 2];

        let mut distance = 0;

        // get row distance
        for r in rows.iter() {
            distance += i32::abs(r - row);
        }

        for c in cols.iter() {
            distance += i32::abs(c - col);
        }

        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_296() {
        assert_eq!(
            Solution::min_total_distance(vec![
                vec![1, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0]
            ]),
            6
        );
    }
}
