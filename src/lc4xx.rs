use std::cmp::max;
use std::ops::Add;
use std::str::FromStr;

pub fn add_strings(num1: String, num2: String) -> String {
    let l1 = num1.as_bytes().len();
    let l2 = num2.as_bytes().len();
    let max_len = max(l1, l2);
    let mut s1 = num1;
    let mut s2 = num2;
    let insert_len = max(max_len - l1, max_len - l2);
    let mut insert_str = String::new();
    for i in 0..insert_len {
        insert_str.push('0');
    }

    if l1 < l2 {
        s1.insert_str(0, &insert_str);
    } else {
        s2.insert_str(0, &insert_str);
    }


    let mut bor: u8 = 0;
    let ascii_0 = '0' as u8;
    let mut res = String::new();
    for i in 0..max_len {
        let index = max_len - 1 - i;
        let n1 = (s1.as_bytes()[index] as u8 - ascii_0);
        let n2 = (s2.as_bytes()[index] as u8 - ascii_0);
        let sum = bor + n1 + n2;
        bor = sum / 10;
        res.push(((sum % 10) + ascii_0) as char)
    }

    if bor != 0 {
        res.push('1');
    }

    let mut c = String::new();
    for i in res.bytes().rev() {
        c.push(char::from(i));
    }

    return c;
}