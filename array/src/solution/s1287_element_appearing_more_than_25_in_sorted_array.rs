#![allow(unused)]
pub struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        // 25% * len 
       let (mut left, mut right) = (0, arr.len() / 4 );
        
        
        // because array is sorted ,so compare duplicate value count 
       while right < arr.len() {
           if arr[left] == arr[right] {
               return arr[left];
           }
           left += 1;
           right += 1;
        }
        
        arr[0]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1287() {
        assert_eq!(
            Solution::find_special_integer(vec![1,2,2,6,6,6,6,7,10]),
            6
        );
    }
}
