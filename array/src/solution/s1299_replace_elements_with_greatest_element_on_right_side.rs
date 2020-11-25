#![allow(unused)]
pub struct Solution {}

// amazon interview
impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        use std::cmp::max;
        // initilize mx = -1 , mx represend the max on the right
        let (mut mx, n, mut tmp) = (-1, arr.len(), 0);
        for i in (0..n).rev() {
            tmp = arr[i];
            arr[i] = mx;
            mx = max(mx, tmp);
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1299() {
    }
}
