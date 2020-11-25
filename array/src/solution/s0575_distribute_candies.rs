#![allow(unused)]
pub struct Solution {}

// microsoft interview
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        // initial hashset filtered duplicate values;
        let (mut set, len) = (
            HashSet::new(),
            if candies.len() % 2 == 0 {
                candies.len() / 2
            } else {
                (candies.len() + 1) / 2
            },
        );
        for i in 0..candies.len() {
            if set.contains(&candies[i]) {
                continue;
            }
            // sister can get max half of candies
            if set.len() == len {
                break;
            }
            // sister get one candy
            set.insert(candies[i]);
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_575() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }
}
