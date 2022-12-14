use std::{collections::VecDeque, ptr::NonNull};


pub fn str2int(s: &String) -> i32 {
    let chars = s.as_bytes();
    ((chars[0] as i32) - 48) * 1000
        + ((chars[1] as i32) - 48) * 100
        + ((chars[2] as i32) - 48) * 10
        + ((chars[3] as i32) - 48)
}


pub fn bfs(target: i32, s: i32, flags: &mut [i32; 10000]) {
    let mut sq = VecDeque::from([s]);
    while !sq.is_empty() {
        let f = sq.front().clone();
        match f {
            Some(seed) => {
                println!("seed = {}", seed);
                let arr: [i32; 4] = [seed / 1000, (seed % 1000) / 100, (seed % 100) / 10, seed % 10];
                let mul: [i32; 4] = [1000, 100, 10, 1];
                for i in 0..arr.len() {
                    let nv1 = (arr[i] + 11) % 10;
                    let nv2 = (arr[i] + 9) % 10;
                    let v1 = seed + (nv1 - arr[i]) * mul[i];
                    let v2 = seed + (nv2 - arr[i]) * mul[i];
                    if v1 == target || v2 == target {
                        println!("find");
                    }
                    if flags[v1 as usize] == 0 {
                        flags[v1 as usize] = 1;
                        sq.push_back(v1);
                    }
                    if flags[v2 as usize] == 0 {
                        sq.push_back(v2);
                    }
                }
            }
            None => { break; }
        }
    }
    //遍历   
}


pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut flags: [i32; 10000] = [0; 10000];
    for i in deadends.iter() {
        flags[str2int(i) as usize] = 1;
    }

    bfs(str2int(&target), 0, &mut flags);
    return 1;
}