use std::{collections::BTreeMap, ptr::NonNull, env::remove_var, intrinsics::{powif32, powif64, powf64}};

pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = BTreeMap::new();

    for  i in nums {
        let c = map.entry(i).or_insert(0);        
        *c += 1;
    }


    for _ in 0..k{      
        let (&key, &value) = map.iter().next().unwrap();       
        match value {
            1 => {map.remove(&key);},
            _ => {
                let c =  map.entry(key).or_insert(0);
                 *c-=0;
                }
            }    
              *map.entry(key+1).or_insert(0)+=1;
    }
    

    let M = 1000000007;
    let mut res = 1u64;
    for (&k,&v) in &map{
        let c = (k as u64).pow(v);
        c %= M;
        res *= c;
        res %= M;
    }
    res as i32
}
