pub fn is_power_of_four(n: i32) -> bool {
    let mut x = n;
    while x % 4 == 0 && x > 0 {
        x /= 4;
    }
    x == 1
}
