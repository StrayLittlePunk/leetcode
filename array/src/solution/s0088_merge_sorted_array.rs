#![allow(unused)]
pub struct Solution {}


impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
      // two get pointers for nums1 and nums2
      let mut p1 = (m -1);
      let mut p2 = (n -1);
      // set pointer from nums1
      let mut p = (m + n -1);


      // while there are still elements to compare
      while p2 >= 0{
        if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
          nums1[p as usize] = nums1[p1 as usize];
          p1 -= 1;
        }else {
          nums1[p as usize] = nums2[p2 as usize];
          p2 -= 1;
        } 
        p -= 1;
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1,vec![1, 2, 2, 3, 5, 6]);
    }
}
