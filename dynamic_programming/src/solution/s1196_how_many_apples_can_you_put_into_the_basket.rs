#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn max_number_of_apples(mut arr: Vec<i32>) -> i32 {
        let mut weight: i32 = 5000;
        arr.sort();
        let mut ans = 0;
        for i in 0..arr.len() {
            if weight > arr[i] {
                ans += 1;
                weight -= arr[i];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1066() {}
}
