#![allow(unused)]
pub struct Solution {}

impl Solution {
     // O(log n) O(1)
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n: i64 = n as i64;
        let mut base: i64 = 9;
        let mut digits: i64 = 1;

        // step 1. calculate how many digits the number has.
        while n - base * digits > 0 {
            n -= base * digits;
            base *= 10;
            digits += 1;
        }

        //  If index equals to 0, it means the target digit is the last digit of number
        let index = (n - 1) % digits;
        let offset = (n - 1) / digits;
        // step 2. calculate what the number is.
        let mut num = i64::pow(10, (digits - 1) as u32) + offset;

        // step 3. find out which digit in the number is we wanted.


        for _ in index+1..digits {
            num /= 10;
        }

        (num % 10) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(Solution::find_nth_digit(3), 3);
        assert_eq!(Solution::find_nth_digit(11), 0);
        assert_eq!(Solution::find_nth_digit(15), 2);
        assert_eq!(Solution::find_nth_digit(250), 1);
        assert_eq!(Solution::find_nth_digit(1000000000), 1);

    }
}
