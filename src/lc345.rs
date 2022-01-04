pub fn is_vowels(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
pub fn reverse_vowels(s: String) -> String {
    let res: String = s.clone();
    let mut f = 0;
    let mut e = res.len() - 1;
    while f < e {
        //  while !is_vowels(&res[f]) {}
    }
    res
}
