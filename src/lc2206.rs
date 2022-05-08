
use std::{collections::HashMap, hash::Hash};
pub fn divide_array(nums: Vec<i32>) -> bool {
    
    let mut m = HashMap::new();
    for n in nums{
        let count = m.entry(n).or_insert(0);
        *count += 1;
    }

    let mut res = true;
    for (_,v) in &m{
        if v % 2 != 0{
            res  =false;
            break;
        }
    }

    res


}
