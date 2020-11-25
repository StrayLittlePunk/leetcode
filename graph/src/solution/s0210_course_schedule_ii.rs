#![allow(unused)]
pub struct Solution {}

use std::collections::HashMap;

#[derive(Clone)]
enum State {
    Unknown,
    Visiting,
    Visited,
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {

        // bound check 
        if num_courses < 1 {
          return vec![];
        }

        // course -> list of next courses
        let mut course_map: HashMap<usize, Vec<usize>> = HashMap::new();

        // build the graph first
        for relation in prerequisites {
            // relation[0] depends on relation[1]
            let edges = course_map
                .entry(relation[1] as usize)
                .or_insert(vec![relation[0] as usize]);
            edges.push(relation[0] as usize);
        }

        let mut state = vec![State::Unknown; num_courses as usize];
        let mut ans = vec![];

        for curr_course in 0..num_courses as usize {
            if Self::is_cyclic(curr_course, &mut state, &course_map, &mut ans) {
                return vec![];
            }
        }

        ans.reverse();
        ans
    }
    fn is_cyclic(
        curr_course: usize,
        state: &mut Vec<State>,
        course_map: &HashMap<usize, Vec<usize>>,
        ans: &mut Vec<i32>,
    ) -> bool {
        match state[curr_course] {
            State::Visiting => true,
            State::Visited => false,
            State::Unknown => {
                state[curr_course] = State::Visiting;

                if let Some(courses) = course_map.get(&curr_course) {
                    // Topological Sort, to visit all its children first.
                    for next_course in courses.iter() {
                        if Self::is_cyclic(*next_course, state, course_map, ans) {
                            return true;
                        }
                    }
                }

                state[curr_course] = State::Visited;
                ans.push(curr_course as i32);

                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_210() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);

        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 2, 1, 3]
        );
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    }
}
