pub fn is_valid_string(s: &String) -> bool {
    s.as_bytes()[0].is_ascii()
}
pub fn first_palindrome(words: Vec<String>) -> String {
    let mut res: String = String::from("");
    for str in words {
        if is_valid_string(str) {
            res = str;
            break;
        }
    }
    res
}
