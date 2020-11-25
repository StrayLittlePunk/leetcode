#![allow(unused)]
pub struct Solution {}

use std::cmp::max;
impl Solution {
    // Char. number range  |        UTF-8 octet sequence
    //     (hexadecimal)    |              (binary)
    //  --------------------+---------------------------------------------
    //  0000 0000-0000 007F | 0xxxxxxx
    //  0000 0080-0000 07FF | 110xxxxx 10xxxxxx
    //  0000 0800-0000 FFFF | 1110xxxx 10xxxxxx 10xxxxxx
    //  0001 0000-0010 FFFF | 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
    //  Time O(N) Space O(1) 
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        // Number of bytes in the current UTF-8 character
        let mut n_bytes = 0;
        //  Mask to check if the most significant bit (8th bit from the left) is set or not
        let mask1 = 1 << 7;

        //  Mask to check if the second most significant bit is set or not
        let mask2 = 1 << 6;
        for num in data {
            let mut mask = 1 << 7;
            if n_bytes == 0 {
                while mask & num != 0{
                    n_bytes += 1;
                    mask = mask >> 1;
                }

                // 1 byte characters
                if n_bytes == 0 {
                    continue;
                }

                // Invalid scenarios according to the rules of the problem.
                if n_bytes == 1 || n_bytes > 4 {
                    return false;
                }
            } else {
                // If this byte is a part of an existing UTF-8 character, then we
                // simply have to look at the two most significant bits and we make
                // use of the masks we defined before.
                // check second 10xxxxxx
                if !((num & mask1) != 0 && !(num & mask2 != 0)) {
                  return false;
                }
                
            }
            n_bytes -= 1;
        }

        n_bytes == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_393() {
        assert_eq!(Solution::valid_utf8(vec![197, 130, 1]), true);
        assert_eq!(Solution::valid_utf8(vec![235, 140, 4]), false);
    }
}
