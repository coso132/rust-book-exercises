fn main() {
    dbg!(max_of_slice(&[34, 50, 225, 100, 65]));
}

fn max_of_slice(v: &[i32]) -> Option<&i32> {
    if v.len() <= 0 {
        return None;
    }
    let mut largest = &v[0];
    for item in v {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
