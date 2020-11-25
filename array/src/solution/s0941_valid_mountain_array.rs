#![allow(unused)]
pub struct Solution {}

// google interview
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        // base case len < 3 would not be a mountain array
        if arr.len() < 3 {
            return false;
        }

        let mut i = 0;
        // walk up
        while i + 1 < arr.len() && arr[i] < arr[i + 1] {
            i += 1;
        }

        // peak can't be first or last
        if i == 0 || i == arr.len() - 1 {
            return false;
        }

        // walk down
        while i + 1 < arr.len() && arr[i] > arr[i + 1] {
            i += 1;
        }

        i == arr.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_941() {
        assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
        assert_eq!(Solution::valid_mountain_array(vec![0,3,2,1]), true);
    }
}
