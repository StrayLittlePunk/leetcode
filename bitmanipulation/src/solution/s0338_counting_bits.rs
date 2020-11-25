#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        // base case num = 1;
        let mut ans = vec![0];
        let mut b = 1;
        let mut i = 0;

        while b <= num {
            while i < b && i + b <=  num {
                ans.push(ans[i as usize] + 1);
                i += 1;
            }

            i = 0;
            b <<= 1;
        }

        ans
    }

    // Time O(N) Space O(N)
    pub fn count_bits_d(num: i32) -> Vec<i32> {
        let mut ans = vec![0; num as usize + 1];

        for i in 1..=num as usize {
            ans[i] = ans[i & (i - 1)] + 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_338() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
