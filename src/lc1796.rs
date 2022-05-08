use std::collections::BTreeSet;
pub fn second_highest(s: String) -> i32 {
    let mut set = BTreeSet::new();
    for i in s.chars() {
        if i >= '0' && i <= '9' {
            let num = (i as i8) - ('0' as i8);
            set.insert(num);
        }
    }
    let res = match set.into_iter().next() {
        None => -1,
        Some(x) => x,
    };
    res as i32
}
