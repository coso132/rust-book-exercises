#[allow(dead_code)]
fn main() {
    println!("{}", vegetables::add2(2));
}

mod vegetables {
    // add code here
    #[allow(dead_code)]
    pub fn add2(n: i32) -> i32 {
        n + 2
    }
}
