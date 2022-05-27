
use std::{collections::HashMap};
pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut prefix = vec![0];
    for i in nums.iter(){
        let last = prefix[prefix.len()-1];
        let ele = match  i {
            1 => 1,
            _ => -1
        };
        prefix.push(ele + last);
    }
    let mut map = HashMap::new();

    let mut max_len:i32 = 0;
    for index in 0..prefix.len(){
        let value = prefix[index];
        let it = map.get(&value);    
        match it {
            None => {map.insert(value, index);},
            Some(value)=>{         
                let dist = (index - value) as i32;
                if max_len < dist  {
                    max_len = dist ;
                }               
            }
        }    
    }
    max_len
}
