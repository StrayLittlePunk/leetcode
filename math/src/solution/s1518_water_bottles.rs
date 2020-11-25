#![allow(unused)]
pub struct Solution {}

// microsoft interview
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        while num_bottles >= num_exchange {
            let empty_bottles = num_bottles / num_exchange;
            ans += empty_bottles;
            num_bottles = empty_bottles + num_bottles % num_exchange;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1518() {}
}
