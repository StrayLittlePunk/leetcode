#![allow(unused)]
pub struct Solution {}

// microsoft interview
//
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        use std::i32::MAX;
        // 1st pass
        let (mut count1, mut count2, mut candi1, mut candi2) = (0, 0, MAX, MAX);
        
        for &n in nums.iter() {
            if candi1 != MAX && candi1 == n {
                count1 += 1;
            } else if candi2 != MAX && candi2 == n {
                count2 += 1;
            } else if count1 == 0 {
                candi1 = n;
                count1 += 1;
            } else if count2 == 0 {
                candi2 = n;
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        
        // 2nd pass
        let mut ret = vec![];
        
        count1 = 0;
        count2 = 0;
        
        for &n in nums.iter() {
            if candi1 != MAX && n == candi1 {
                count1 += 1;
            }
            if candi2 != MAX && n == candi2 {
                count2 += 1;
            }
        }
        
        if count1 > nums.len() / 3 {
            ret.push(candi1);
        }
        if count2 > nums.len() / 3 {
            ret.push(candi2);
        }
        
        ret
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_229() {
    }
}
