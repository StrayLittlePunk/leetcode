#![allow(unused)]
fn merge(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    if vec1.len() == 0 {
        return vec2;
    }
    if vec2.len() == 0 {
        return vec1;
    }
    let (mut ans, mut i, mut j) = (vec![], 0, 0);
    while i < vec1.len() && j < vec2.len() {
        if vec1[i] < vec2[j] {
            ans.push(vec1[i]);
            i += 1;
        } else {
            ans.push(vec2[j]);
            j += 1;
        }
    }
    // vec1 have more elements
    while i < vec1.len() {
        ans.push(vec1[i]);
        i += 1;
    }
    // vec2 have more elements
    while j < vec2.len() {
        ans.push(vec2[j]);
        j += 1;
    }

    ans
}
