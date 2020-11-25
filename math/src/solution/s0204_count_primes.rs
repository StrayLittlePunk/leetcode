#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut is_prime = vec![true; n as usize];
        let sqrt_n = f32::sqrt(n as f32) as usize;

        for i in 2..=sqrt_n {
            if !is_prime[i] {
                continue;
            }
            let mut k = i * i;
            while k < n as usize {
                is_prime[k] = false;
                k += i
            }
        }

        let mut count = 0;
        for i in 2..n as usize {
            if is_prime[i] {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_367() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
    }
}
