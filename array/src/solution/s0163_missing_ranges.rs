
#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/missing-ranges/
// reference: https://leetcode.com/problems/missing-ranges/discuss/329744/Rust-O(N)-0ms
impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let mut ret = vec![];
        let (lower, upper) = (lower as i64, upper as i64);
        let mut pre = lower - 1;

        for i in nums {
            let i = i as i64;
            if pre + 1 < i {
                ret.push(Self::get_range(pre, i));
            }
            pre = i;
        }
        if pre != upper {
            ret.push(Self::get_range(pre, upper + 1));
        }
        ret
    }

    fn get_range(start: i64, end: i64) -> String {
        let mut interval = (start+1).to_string();
        if start + 2 < end {
            interval += &("->".to_string() + &(end-1).to_string());
        }
        interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_163() {
        assert_eq!(
          Solution::find_missing_ranges(vec![0, 1, 3, 50, 75], 0, 99),
          vec!["2", "4->49", "51->74", "76->99"]);
    }
}
