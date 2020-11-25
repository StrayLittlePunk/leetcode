#![allow(unused)]
pub struct Solution {}

impl Solution {

  // O(n^2) O(n)
    pub fn get_permutation(n: i32, k: i32) -> String {
        let (mut nums, mut factorials, n, mut k) = (vec!['1'], vec![1], n as usize, k as usize);
        for i in 1..n {
            factorials.push(factorials[i - 1] * i);
            nums.push(std::char::from_digit(i as u32 + 1, 10).unwrap());
        }
        k -= 1;

        let mut ans = String::from("");
        for i in (0..n).rev() {
            let idx = k / factorials[i];
            k -= idx * factorials[i];

            ans.push(nums[idx]);
            nums.remove(idx);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
        assert_eq!(Solution::get_permutation(3, 5), "312".to_owned());
        assert_eq!(Solution::get_permutation(3, 3), "213".to_owned());
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_owned());
        assert_eq!(Solution::get_permutation(3, 1), "123".to_owned());
    }
}
