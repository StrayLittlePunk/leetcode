#![allow(unused)]

pub struct Solution {}
use std::cmp::{min, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    val: i32,
    row: usize,
    col: usize,
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let m = matrix.len();

        let mut heap = BinaryHeap::with_capacity(k as usize);

        for r in 0..min(m, k as usize) {
            // we add triplets of information for each cell
            heap.push(Reverse(Point {
                val: matrix[r][0],
                row: r,
                col: 0,
            }));
        }

        // extract-min
        while let Some(Reverse(node)) = heap.pop() {
            let (r, c) = (node.row, node.col);
            if k == 1 {
                return node.val;
            }

            if c < m - 1 {
                heap.push(Reverse(Point {
                    val: matrix[r][c + 1],
                    row: r,
                    col: c + 1,
                }));
            }

            k -= 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_378() {}
}
