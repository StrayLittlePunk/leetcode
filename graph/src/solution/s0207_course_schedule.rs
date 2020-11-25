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
    // 	Time Limit Exceeded
    // Time O(E + V^2) Space O(E + V)
    pub fn can_finish_backtracking(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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

        let mut path = vec![false; num_courses as usize];

        for curr_course in 0..num_courses as usize {
            if Self::is_cyclic_backtracking(curr_course, &course_map, &mut path) {
                return false;
            }
        }

        true
    }

    /*
     * backtracking method to check that no cycle would be formed starting from currCourse
     */

    fn is_cyclic_backtracking(
        curr_course: usize,
        course_map: &HashMap<usize, Vec<usize>>,
        path: &mut Vec<bool>,
    ) -> bool {
        if path[curr_course] {
            // come across a previously visited node, i.e. detect the cycle
            return true;
        }

        // no following courses, no loop
        if !course_map.contains_key(&curr_course) {
            return false;
        }

        // before backtracking, mark the node in the path
        path[curr_course] = true;

        // backtracking
        let mut ret = false;

        for next_course in course_map.get(&curr_course).unwrap().iter() {
            ret = Self::is_cyclic_backtracking(*next_course, course_map, path);
            if ret {
                break;
            }
        }

        // after backtracking, remove the node from the path
        path[curr_course] = false;

        ret
    }

    // Time O(E + V) Space O(E + V)
    pub fn can_finish_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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

        let mut path = vec![false; num_courses as usize];
        let mut checked = vec![false; num_courses as usize];

        for curr_course in 0..num_courses as usize {
            if Self::is_cyclic_dfs(curr_course, &course_map, &mut path, &mut checked) {
                return false;
            }
        }

        true
    }

    /*
     * postorder DFS check that no cycle would be formed starting from currCourse
     */
    fn is_cyclic_dfs(
        curr_course: usize,
        course_map: &HashMap<usize, Vec<usize>>,
        path: &mut Vec<bool>,
        checked: &mut Vec<bool>,
    ) -> bool {
        // bottom cases
        if checked[curr_course] {
            // this node has been checked, no cycle would be formed with this node.
            return false;
        }

        if path[curr_course] {
            // come across a previously visited node, i.e. detect the cycle
            return true;
        }

        // no following courses, no loop
        if !course_map.contains_key(&curr_course) {
            return false;
        }

        // before backtracking, mark the node in the path
        path[curr_course] = true;

        // backtracking
        let mut ret = false;

        // postorder DFS, to visit all its children first.
        for next_course in course_map.get(&curr_course).unwrap().iter() {
            ret = Self::is_cyclic_dfs(*next_course, course_map, path, checked);
            if ret {
                break;
            }
        }

        // after the visits of children, we come back to process the node itself
        // remove the node from the path
        path[curr_course] = false;

        // Now that we've visited the nodes in the downstream,
        // we complete the check of this node.
        checked[curr_course] = true;

        ret
    }
    // Time O(E + V) Space O(E + V) Topological Sort
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {

        // bound check 
        if num_courses < 1 {
          return false;
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

        for curr_course in 0..num_courses as usize {
            if Self::is_cyclic(curr_course, &mut state, &course_map) {
                return false;
            }
        }

        true
    }

    fn is_cyclic(
        curr_course: usize,
        state: &mut Vec<State>,
        course_map: &HashMap<usize, Vec<usize>>,
    ) -> bool {
        match state[curr_course] {
            State::Visiting => true,
            State::Visited => false,
            State::Unknown => {
                state[curr_course] = State::Visiting;

                if let Some(courses) = course_map.get(&curr_course) {
                    // Topological Sort, to visit all its children first.
                    for next_course in courses.iter() {
                        if Self::is_cyclic(*next_course, state, course_map) {
                            return true;
                        }
                    }
                }

                state[curr_course] = State::Visited;

                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);

        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }
}
