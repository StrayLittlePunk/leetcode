#![allow(unused)]
pub struct Solution {}

// facebook interview
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            2 => 1,
            n => {
                let (mut a, mut b, mut c) = (0, 1, 1);
                for i in 3..=n {
                    let tmp = a + b + c;
                    a = b;
                    b = c;
                    c = tmp;
                }

                c
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1137() {}
}
