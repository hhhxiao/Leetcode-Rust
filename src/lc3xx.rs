//387
use std::collections::HashMap;
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
