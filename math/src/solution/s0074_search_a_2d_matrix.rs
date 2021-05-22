#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(log(mn)) O(1)
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut start, mut end) = (0, m * n - 1);
        while start + 1 < end {
            let mid = start + (end - start) / 2;
            let (i, j) = (mid / n, mid % n);
            if matrix[i][j] < target {
                start = mid;
            } else if matrix[i][j] > target {
                end = mid;
            } else {
                return true;
            }
        }

        matrix[start / n][start % n] == target || matrix[end / n][end % n] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_74() {
        // show stdout message
        //$: cargo t -- --test-threads=1 --nocapture
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            ),
            false
        );
        assert_eq!(Solution::search_matrix(Vec::<Vec<i32>>::new(), 0), false);
        assert_eq!(Solution::search_matrix(vec![vec![1]], 0), false);
    }
}
