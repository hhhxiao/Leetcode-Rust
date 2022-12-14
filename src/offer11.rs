pub fn min_array(numbers: Vec<i32>) -> i32 {
    let res = match numbers.iter().min() {
        Some(x) => *x,
        None => -1
    };
    res
}