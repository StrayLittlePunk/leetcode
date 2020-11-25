#![allow(unused)]
pub struct Solution {}

// apple interview

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        // two pointer quicksoft partition algorithm
        let (mut left, mut right) = (0, a.len() - 1);
        while left < right {
            // a[left] is odd number and a[right] is even number , exchange them
            if a[left] % 2 > a[right] % 2 {
                a.swap(left, right);
            }

            // a[left] is even number , keep going
            if a[left] % 2 == 0 {
                left += 1;
            }
            // a[right] is odd number , keep going
            if a[right] % 2 == 1 {
                right -= 1;
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_905() {}
}
