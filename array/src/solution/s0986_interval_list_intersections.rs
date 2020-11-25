#![allow(unused)]
pub struct Solution {}

// facebook interview
impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::{max, min};
        let (mut ans, mut i, mut j) = (vec![], 0, 0);

        while i < a.len() && j < b.len() {

            // Let's check if A[i] intersects B[j].
            // lo - the startpoint of the intersection
            // hi - the endpoint of the intersection
            let lo = max(a[i][0], b[j][0]);
            let hi = min(a[i][1], b[j][1]);
            if lo <= hi {
                ans.push(vec![lo, hi]);
            }

            // Remove the interval with the smallest endpoint
            if a[i][1] < b[j][1] {
                i += 1;
            } else {
                j += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_986() {}
}
