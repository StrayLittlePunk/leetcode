#![allow(unused)]

pub struct Solution {}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

// (next_cost, next_destination, current_steps)
#[derive(Eq)]
struct Flight(i32, i32, i32);

impl Ord for Flight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Flight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

const SRC: usize = 0;
const DST: usize = 1;
const COST: usize = 2;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        flights.iter().for_each(|flight| {
            graph
                .entry(flight[SRC])
                .and_modify(|e| e.push((flight[DST], flight[COST])))
                .or_insert(vec![(flight[DST], flight[COST])]);
        });

        let mut heap: BinaryHeap<Flight> = BinaryHeap::new();
        heap.push(Flight(0, src, -1));

        while !heap.is_empty() {
            if let Some(Flight(next_cost, next_dst, curr_steps)) = heap.pop() {
                if curr_steps > k { continue; }
                if next_dst == dst { return next_cost; }

                for (nd, nc) in graph.get(&next_dst).unwrap_or(&Vec::new()).iter() {
                    heap.push(Flight(next_cost + nc, *nd, curr_steps + 1));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_787() {
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1
            ),
            200
        );
        assert_eq!(
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0
            ),
            500
        );
    }
}
