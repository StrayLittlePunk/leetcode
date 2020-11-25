#![allow(unused)]
pub struct Solution {}

// facebook interview
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        fn count_le(
            mat: &Vec<Vec<i32>>,
            m: usize,
            n: usize,
            target: i32,
            r: usize,
            sum: i32,
            k: i32,
        ) -> i32 {
            if sum > target {
                return 0;
            }
            if r == m {
                return 1;
            }
            let mut ans = 0;
            for c in 0..n {
                let cnt = count_le(mat, m, n, target, r + 1, sum + mat[r][c], k - ans);
                if cnt == 0 {
                    break;
                }
                ans += cnt;
                if ans > k {
                    break;
                } // prune when count > k
            }
            ans
        }
        let m = mat.len();
        let n = mat[0].len();
        let mut left = m;
        let mut right = m * 5000;
        let mut ans = 0;
        while left <= right {
            let mid = left + (right - left) / 2;
            let cnt = count_le(&mat, m, n, mid as i32, 0, 0, k);
            if cnt >= k {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1439() {
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 3, 11], vec![2, 4, 6]], 5),
            7
        );
    }
}
