#![allow(unused)]
pub struct Solution {}

// google interview
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::max;
        // 从 x 移动到 y 的最少次数为 dx 和 dy 中的较大值 max(dx, dy)，
        // 这也被称作 x 和 y 之间的 切比雪夫距离。
        let mut ans = 0;

        for i in 1..points.len() {
            ans += max(
                i32::abs(points[i][0] - points[i - 1][0]),
                i32::abs(points[i][1] - points[i - 1][1]),
            );
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1266() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
    }
}
