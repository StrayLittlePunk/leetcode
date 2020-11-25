#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::usize::MAX;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    val: i32,
    row: usize,
    col: usize,
}

impl Solution {
    //  let X= min(K, N); X+Klog(X) O(X)
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
    pub fn kth_smallest_binary_search(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let n = matrix.len();
        let (mut start, mut end) = (matrix[0][0], matrix[n - 1][n - 1]);

        while start < end {
            let mid = start + (end - start) >> 1;

            let (count, smaller, larger) =
                Self::count_less_equal(&matrix, mid, matrix[0][0], matrix[n - 1][n - 1]);

            if count == k {
                return smaller;
            }
            if count < k {
                start = larger;
            } else {
                end = smaller;
            }
        }

        start
    }

    fn count_less_equal(
        matrix: &Vec<Vec<i32>>,
        mid: i32,
        mut smaller: i32,
        mut larger: i32,
    ) -> (i32, i32, i32) {
        let (mut count, n) = (0, matrix.len());
        let (mut r, mut c) = (n as i32 - 1, 0);

        while r >= 0 && c < n {
            if matrix[r as usize][c] > mid {
                // As matrix[r][c] is bigger than the mid, let's keep track of the
                // smallest number greater than the mid
                larger = min(larger, matrix[r as usize][c]);
                r -= 1;
            } else {
                // As matrix[r][c] is less than or equal to the mid, let's keep track of the
                // biggest number less than or equal to the mid
                smaller = max(smaller, matrix[r as usize][c]);
                count += r + 1;
                c += 1;
            }
        }

        (count, smaller, larger)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_378() {
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
    }
}
