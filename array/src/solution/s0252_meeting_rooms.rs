#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/meeting-rooms/solution/

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() < 2 {
            return true;
        }
        let mut copy_intervals = intervals.to_vec();
        copy_intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

        for i in 0..(copy_intervals.len() - 1) {
            if copy_intervals[i][1] > copy_intervals[i + 1][0] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_252() {
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            false
        );
        assert_eq!(
            Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4]]),
            true
        );
    }
}
