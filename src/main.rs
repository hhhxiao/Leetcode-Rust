mod lc2xx;

fn main() {
    let mut v = vec![1, 3, 5, 6, 8, 9, 11, 12];
    let x = lc2xx::summary_ranges(v);
    for i in x {
        println!("{}", i)
    }
}

