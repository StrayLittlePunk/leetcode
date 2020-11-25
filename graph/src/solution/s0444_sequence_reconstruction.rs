#![allow(unused)]

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        let (mut graph, mut indegrees) = (HashMap::new(), HashMap::new());

        for seq in seqs.iter() {
            for num in seq.iter() {
                graph.insert(*num, vec![]);
                indegrees.insert(*num, 0);
            }
        }

        for seq in seqs.iter() {
            for i in 0..seq.len() - 1 {
                let mut neighbors = graph.get(&seq[i]).unwrap().to_vec();
                neighbors.push(seq[i + 1]);
                graph.insert(seq[i], neighbors);
                indegrees.insert(seq[i + 1], indegrees.get(&seq[i + 1]).unwrap() + 1);
            }
        }

        let mut queue = vec![];
        for (node, count) in indegrees.iter() {
            if *count == 0 {
                queue.push(*node);
            }
        }

        let mut res = vec![];
        while queue.len() > 0 {
            if queue.len() > 1 {
                return false;
            }

            let node = queue.remove(0);
            res.push(node);

            let neighbors = graph.get(&node).unwrap();

            for neighbor in neighbors.iter() {
                let indegree = *indegrees.get(neighbor).unwrap() - 1;
                indegrees.insert(*neighbor, indegree);
                if indegree == 0 {
                    queue.push(*neighbor);
                }
            }
        }

        return res.len() == indegrees.len() && Self::vec_compare(&res, &org);
    }

    fn vec_compare(a: &Vec<i32>, b: &Vec<i32>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_444() {
        assert_eq!(
            Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 2], vec![1, 3]]),
            false
        );
        assert_eq!(
            Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 2]]),
            false
        );
        assert_eq!(
            Solution::sequence_reconstruction(
                vec![1, 2, 3],
                vec![vec![1, 2], vec![1, 3], vec![2, 3]]
            ),
            true
        );
        assert_eq!(
            Solution::sequence_reconstruction(
                vec![4, 1, 5, 2, 6, 3],
                vec![vec![5, 2, 6, 3], vec![4, 1, 5, 2]]
            ),
            true
        );
    }
}
