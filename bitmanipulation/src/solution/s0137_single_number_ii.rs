#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Time O(N), Space O(1) simliar to problem 389,  Analysis see problem 389
    // To separate number that appears once from a number that appears three times
    // let's use two bitmasks instead of one: seen_once and seen_twice.
    //  The idea is to
    //  change seen_once only if seen_twice is unchanged
    //  change seen_twice only if seen_once is unchanged

    pub fn single_number(nums: Vec<i32>) -> i32 {
        // first appearence:
        // add num to seen_once
        // don't add to seen_twice because of presence in seen_once

        // second appearance:
        // remove num from seen_once
        // add num to seen_twice

        // third appearance:
        // don't add to seen_once because of presence in seen_twice
        // remove num from seen_twice
        let mut seen_once = 0;
        let mut seen_twice = 0;

        for num in nums {
            seen_once = !seen_twice & (seen_once ^ num);
            seen_twice = !seen_once & (seen_twice ^ num);
        }

        seen_once
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_137() {
        assert_eq!(Solution::single_number(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
