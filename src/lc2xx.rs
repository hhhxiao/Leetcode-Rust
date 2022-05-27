pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut res = Vec::new();
    if nums.is_empty() {
        return res;
    }

    let mut begin: i32 = nums[0];
    let mut end: i32 = nums[0];
    for i in 1..nums.len() {
        let v: i32 = nums[i];
        if v == end + 1 {
            end = v;
        } else {
            if begin == end {
                res.push(begin.to_string());
            } else {
                res.push(begin.to_string() + "->" + &end.to_string());
            }

            begin = v;
            end = v;
        }
    }

    if begin == end {
        res.push(begin.to_string());
    } else {
        res.push(begin.to_string() + "->" + &end.to_string());
    }
    res
}
