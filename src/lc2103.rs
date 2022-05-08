use std::{collections::HashMap};

pub fn count_points(rings: String) -> i32 {
    let mut m   = HashMap::new();
    let ring_num = rings.len()/2;
    for i in 0..ring_num{
        let color = rings.chars().nth(i * 2).unwrap();
        let index = rings.chars().nth(i * 2+1).unwrap();
        let v = m.entry(index).or_insert([0,0,0]);
        match color {
            'R' => v[0]+=1,
            'G' => v[1]+=1,
            'B' => v[2]+=1,       
            _ => ()
        }
    }

    let mut res = 0;
    for (_,v) in &m{
        if v[0] >0 && v[1] > 0 && v[2] >0 {
            res += 1;
        }
    }
    res
}