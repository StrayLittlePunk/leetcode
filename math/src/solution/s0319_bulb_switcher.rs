#![allow(unused)]
pub struct Solution {}

impl Solution {
  // A bulb ends up on iff it is switched an odd number of times.
  // Call them bulb 1 to bulb n. Bulb i is switched in round d if and only if d
  // divides i. So bulb i ends up on if and only if it has an odd number of divisors.
  // Divisors come in pairs, like i=12 has divisors 1 and 12, 2 and 6, and 3 and 4.
  // Except when i is a square, like 36 has divisors 1 and 36, 2 and 18, 3 and 12, 4 and 9, 
  // and double divisor 6. So bulb i ends up on if and only if i is a square.
  // So just count the square numbers.
  // Let R = int(sqrt(n)). That's the root of the largest square in the range [1,n].
  // And 1 is the smallest root. So you have the roots from 1 to R, that's R roots. 
  // Which correspond to the R squares. So int(sqrt(n)) is the answer. (C++ does the conversion
  // to int automatically, because of the specified return type).
    // O(1) O(1)
        pub fn bulb_switch(n: i32) -> i32 {

          f64::sqrt(n as f64) as i32

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_319() {
        assert_eq!(Solution::bulb_switch(3), 1);
    }
}
