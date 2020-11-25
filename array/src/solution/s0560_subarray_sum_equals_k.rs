#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut count = 0;
        let mut cursum = 0;
        for &num in nums.iter() {
            // current prefix sum
            cursum += num;

            // case 1
            // continuous subarray starts
            // from the beginning of the array
            if cursum == k {
                count += 1;
            }

            // case 2
            // number of times the curr_sum − k has occured already,
            // determines the number of times a subarray with sum k
            // has occured upto the current index
            count += map.get(&(cursum - k)).cloned().unwrap_or(0);

            *map.entry(cursum).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_560() {}
}
