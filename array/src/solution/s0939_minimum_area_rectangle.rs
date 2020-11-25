#![allow(unused)]
pub struct Solution {}

// google interview
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        use std::collections::HashSet;

        let mut visited = HashSet::<(i32, i32)>::new();
        let mut ans = std::i32::MAX;
        for p in &points {
            for &(x2, y2) in &visited {
                if visited.contains(&(p[0], y2)) && visited.contains(&(x2, p[1])) {
                    let area = ((p[0] - x2) * (p[1] - y2)).abs();
                    ans = min(area, ans);
                }
            }
            visited.insert((p[0], p[1]));
        }

        if ans == std::i32::MAX {
            0
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_939() {
        assert_eq!(
            Solution::min_area_rect(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![2, 2]
            ]),
            4
        );
        assert_eq!(
            Solution::min_area_rect(vec![
                vec![1, 1],
                vec![1, 3],
                vec![3, 1],
                vec![3, 3],
                vec![4, 1],
                vec![4, 3]
            ]),
            2
        );
    }
}
