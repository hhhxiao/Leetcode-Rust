
use std::collections::HashSet;
pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    
    if original == 0{
        return 0;
    }
    let mut o = original;
    let mut s = HashSet::new();   
    for i in 0..nums.len(){
        s.insert(nums[i]);
    }
    
    while s.contains(&o) {
        o *=2;
    }
    original
    }