#![allow(unused)]
pub struct Solution {}

// google interview
use std::cmp::max;
impl Solution {
    pub fn total_fruit(tree: Vec<i32>) -> i32 {
        let (mut ret, mut cur, mut count_b, mut a, mut b) = (0, 0, 0, 0, 0);
        for &c in tree.iter() {
            cur = if c == a || c == b {
                cur + 1
            } else {
                count_b + 1
            };
            count_b = if c == b { count_b + 1 } else { 1 };
            if b != c {
                a = b;
                b = c;
            }
            ret = max(ret, cur);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_904() {}
}
