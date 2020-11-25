#![allow(unused)]
pub struct Solution {}

impl Solution {
    // O(n) O(1)
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut cur = 1;
        for i in 1..=n {
            ans.push(cur);
            if cur * 10 <= n {
                cur *= 10;
            } else if cur % 10 != 9 && cur + 1 <= n {
                cur += 1;
            } else {
                while (cur / 10) % 10 == 9 {
                    cur /= 10;
                }

                cur = cur / 10 + 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_202() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
    }
}
