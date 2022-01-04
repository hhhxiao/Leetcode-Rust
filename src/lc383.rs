use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut table = HashMap::new();
    for i in magazine.chars() {
        let count = table.entry(i).or_insert(0);
        *count += 1;
    }
    for i in ransom_note.chars() {
        let count = table.entry(i).or_insert(0);
        *count -= 1;
    }
    println!("{:?}", table);
    let mut res = true;
    for (_, value) in table {
        if value < 0 {
            res = false;
            break;
        }
    }
    res
}
