pub fn reverse_string(s: &mut Vec<char>) {
    for i in 0..(s.len() + 1) / 2 {
        let idx = s.len() - i - 1;
        let c = s[i];
        s[i] = s[idx];
        s[idx] = c;
    }
}
