#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap().to_digit(10).unwrap() as i32
    }
}

#[test]
fn testcase() {
    assert_eq!(Solution::min_partitions("32".into()), 3);
    assert_eq!(Solution::min_partitions("82734".into()), 8);
    assert_eq!(Solution::min_partitions("27346209830709182346".into()), 9);
}
