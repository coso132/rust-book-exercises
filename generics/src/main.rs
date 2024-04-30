use std::cmp::Ord;
fn main() {
    dbg!(largest_or_panic(&[
        1, 7, 3, 5, 7, 0, 5, 3, 7, 913, 51236, 75, 2
    ]));
    dbg!(largest_or_panic(&['d', 'e', 'f', 'a', 'h', 'q', 'j']));
}

fn largest_or_panic<T: Ord>(ls: &[T]) -> &T {
    ls.iter().max().unwrap()
}
