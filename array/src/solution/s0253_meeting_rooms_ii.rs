#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/meeting-rooms-ii/

// Chronological Ordering
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() < 1 {
            return 0;
        }
       let mut start = vec![0; intervals.len()];
       let mut end = vec![0; intervals.len()];

       for i in 0..intervals.len() {
         start[i] = intervals[i][0];
         end[i] = intervals[i][1];
       }

       // Sort the intervals by end time
      end.sort_by(|a, b| a.partial_cmp(&b).unwrap());
       // Sort the intervals by start time
      start.sort_by(|a, b| a.partial_cmp(&b).unwrap());
      // the two pointers in the algorithm: e_ptr and s_ptr
      let mut s_ptr =0;
      let mut e_ptr =0;

      // variables to keep track of maximum number of rooms used
      let mut used_rooms = 0;
      while s_ptr < intervals.len() {
        // if there is a meeting that has ended by the time the meeting 
        // at `s_ptr` starts 
        if start[s_ptr] >= end[e_ptr] {
          used_rooms -= 1;
          e_ptr += 1;
        }
        // We do this irrespective of whether a room frees up or not.
        // If a room got free, then this used_rooms += 1 wouldn't have 
        // any effect. used_rooms would remain the same in that case. 
        // If no room was free, then this would increase used_rooms 
        used_rooms += 1;
        s_ptr += 1
      }
      used_rooms 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_253() {
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![0, 30], vec![5, 10], vec![15, 20]]),
            2
        );
        assert_eq!(
            Solution::min_meeting_rooms(vec![vec![7, 10], vec![2, 4]]),
            1
        );
    }
}
