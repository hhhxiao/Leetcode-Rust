use std::collections::HashMap;

pub fn find_string(words: Vec<String>, s: String) -> i32 {
    let mut map = HashMap::new();
    for (idx, v) in words.iter().enumerate() {
        map.insert(v, idx)
    }

    1
}