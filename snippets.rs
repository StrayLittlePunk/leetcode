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

// or mean buddle sort
fn insert_sort(nums: &mut Vec<i32>) {

    for i in 1..nums.len() {
        let mut j = i;
        while j > 0 && nums[j] < nums[j - 1] {
            nums.swap(j , j - 1);
            j -= 1;
        }
    }

    // improve version 
    // 因为内循环总是给变量t赋同样的值（x[i]的初始值），所以我们可以t 的赋值语句移出
    // 内循环，修改比较语句 
    for i in 1..nums.len() {
        let mut j = i;
        let t = nums[i];
        while j > 0 && nums[j - 1] > t {
            nums[j] = nums[j - 1];
            j -= 1;
        }
        nums[j] = t;
    }
}

// find prime factor number
fn solution(mut num: i64) {
    // Print the number of 2s that divide n
   while num % 2 == 0 {
       print!("{} ", 2);
       num /= 2;
    }

    // n must be odd at this point.  So we can skip
    // one element (Note i = i +2)
    for i in (3..=(num as f64).sqrt() as i64).step_by(2) {
        while num % i == 0 {
            print!("{} ", i);
            num = num / i;
        }
    }

    // This condition is to handle the case when n
    // is a prime number greater than 2
    if num > 2 {
       print!("{} ", num);
    }

}
