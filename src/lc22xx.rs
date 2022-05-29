pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    let mut sum = 0;
    for i in words {
        if s.starts_with(&i) {
            sum += 1
        }
    }
    sum
}