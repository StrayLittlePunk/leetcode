#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(log(mn)) O(1)
    pub fn search_matrix_my(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 {
            return false;
        }

        let n = matrix[0].len();
        if n == 0 {
            return false;
        }

        // row search
        let (mut start, mut end) = (0, m as i32 - 1);
        while start <= end {
            let mid = start + (end - start >> 1);

            if matrix[mid as usize][0] == target {
                return true;
            } else if matrix[mid as usize][0] > target {
                end = mid - 1;
            } else if matrix[mid as usize][0] < target {
                start = mid + 1;
            }
        }

        // column search
        if start > 0 {
            start -= 1;
        }
        let (mut c1, mut c2) = (0, n as i32 - 1);

        while c1 <= c2 {
            let mid = c1 + (c2 - c1 >> 1);
            if matrix[start as usize][mid as usize] == target {
                return true;
            } else if matrix[start as usize][mid as usize] > target {
                c2 = mid - 1;
            } else if matrix[start as usize][mid as usize] < target {
                c1 = mid + 1;
            }
        }

        false
    }

    // O(log(mn)) O(1)
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        if m == 0 {
            return false;
        }

        let n = matrix[0].len();

        // binary search
        let (mut start, mut end) = (0, m as i32 - 1);
        while start <= end {
            let mid = (start + (end - start >> 1)) as usize;

            if matrix[mid / m][mid % n] == target {
                return true;
            } else if matrix[mid / m][mid % n] > target {
                end = mid as i32 - 1;
            } else if matrix[mid / m][mid % n] < target {
                start = mid as i32 + 1;
            }
        }

        false
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
