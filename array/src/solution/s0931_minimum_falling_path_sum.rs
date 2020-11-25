#![allow(unused)]
pub struct Solution {}

// apple interview
impl Solution {
    pub fn min_falling_path_sum(mut a: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;
        use std::i32::MAX;
        let len = a.len();
        for r in (0..len-1).rev() {
            for c in 0..len {
                // best = min(a[r+1][c-1], a[r+1][c], a[r+1][c+1])
                let mut best = a[r+1][c];
                if c > 0 {
                    best = min(best, a[r+1][c-1]);
                }
                if c+1 < len {
                    best = min(best, a[r+1][c+1]);
                }
                a[r][c] += best;
            }
        }
        
        let mut ans = MAX;
        for &x in a[0].iter() {
            ans = min(ans, x);
        }
        
        ans
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_931() {
    }
}
