#![allow(unused)]
pub struct Solution {}

impl Solution {
    // Min Heap Time O(k log k) Space O(k)
    pub fn get_skyline(mut buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::BinaryHeap;
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(buildings.len());
        let mut valid = vec![false; buildings.len()];
        let mut line: Vec<(i32, i32, bool, i32)> = Vec::with_capacity(2 * buildings.len());
        let mut res: Vec<Vec<i32>> = vec![];

        for (i, data) in buildings.iter().enumerate() {
            line.push((data[0], i as i32, true, data[2]));
            line.push((data[1], i as i32, false, data[2]));
        }

        // Sort by coordinate , untie by larger height
        line.sort_unstable_by_key(|x| (x.0, -(x.3 as i64)));

        for point in line {
            let coord = point.0;
            let i = point.1;
            let is_start = point.2;

            if is_start {
                let height = buildings[i as usize][2];
                valid[i as usize] = true;
                heap.push((height, i));
                res.push(vec![coord, heap.peek().unwrap().0]);
            } else {
                valid[i as usize] = false;
                while let Some(&(_, j)) = heap.peek() {
                    if valid[j as usize] {
                        break;
                    }
                    heap.pop();
                }
                res.push(vec![coord, heap.peek().unwrap_or(&(0, 0)).0]);
            }
        }

        // Remove all entries with the same coordinate but one 
        // (keep the one that appears last in the list)
        let mut last_coord = -1;
        res = res.into_iter().rev().filter(|x| {
          if x[0] != last_coord {
            last_coord = x[0];
            return true;
          }
          return false;
        }).collect::<Vec<Vec<i32>>>();

        // Remove all entries with the same height but one 
        // (keep the one that appears first in the list)
        let mut h = -1;
        res = res.into_iter().rev().filter(|x|{
          if x[1] != h {
            h = x[1];
            return true;
          }
          return false;
        }).collect();

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_218() {
        assert_eq!(
            Solution::get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ]),
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ]
        );
    }
}
