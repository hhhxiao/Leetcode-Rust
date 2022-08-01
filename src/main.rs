extern crate core;

mod lc15xx;
mod lc4xx;
mod lc3xx;
mod lc21xx;

fn main() {
    let str = String::from("hello world");
    lc21xx::capitalize_title(str);
}
