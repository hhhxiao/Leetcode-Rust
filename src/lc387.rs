use std::collections::HashMap;
pub fn first_uniq_char(s: String) -> i32 {
    let mut m = HashMap::new();
    for i in s.chars() {
        let stat = m.entry(i).or_insert(0);
        *stat += 1;
    }

    print!("{:?}", m);
    for (key, value) in &m {}
    12
}
