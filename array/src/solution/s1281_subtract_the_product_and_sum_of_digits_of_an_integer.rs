#![allow(unused)]
pub struct Solution {}

// uber interview

impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let (mut product, mut sum) = (1, 0);
        while n > 0 {
            let rem = n % 10;
            product *= rem;
            sum += rem;
            n /= 10;
        }

        product - sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1281() {}
}
