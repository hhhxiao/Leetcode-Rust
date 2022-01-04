pub fn is_ugly(n: i32) -> bool {
    let mut x = n;
    while x % 2 == 0 && x > 0 {
        x /= 2;
    }
    while x % 5 == 0 && x > 0 {
        x /= 5;
    }
    while x % 3 == 0 && x > 0 {
        x /= 3;
    }
    x == 1
}
