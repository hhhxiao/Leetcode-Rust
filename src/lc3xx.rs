//387
use std::collections::{HashMap, HashSet};

pub fn first_uniq_char(s: String) -> i32 {
    let mut m = HashMap::new();
    for i in s.as_bytes() {
        let count = m.entry(i).or_insert(0);
        *count += 1;
    }

    for i in 0..s.as_bytes().len() {
        let res = m.get(&s.as_bytes()[i]);
        match res {
            Some(cnt) => {
                if *cnt == 2 {
                    return i as i32;
                }
            }
            None => (),
        }
    }
    return -1;
}

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    let mut res: Vec<i32> = vec![];

    nums1.iter().for_each(|x| { s1.insert(*x); });
    nums2.iter().for_each(|x| { s2.insert(*x); });
    s1.iter().for_each(|x| {
        match s2.get(x) {
            None => (),
            Some(x) => { res.push(*x) }
        }
    });
    res
}