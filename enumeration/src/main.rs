#![allow(dead_code)]
#![allow(unused)]

#[derive(Debug)]
enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let four = Ip::V4(127, 0, 0, 1);
    let six = Ip::V6(String::from(""));
    dbg!(four);
    dbg!(six);
}
