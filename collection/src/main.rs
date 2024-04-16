use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 8, 8];
    dbg!(median_mode(&v));
}

fn median_mode(v: &Vec<i32>) -> Option<(i32, i32)> {
    Some((
        match mediam(v) {
            Some(x) => x,
            None => return None,
        },
        mode(v).unwrap(),
    ))
}

fn mediam(v: &Vec<i32>) -> Option<i32> {
    match v.len() {
        0 => None,
        l => {
            let mut r = v.clone();
            r[0..l / 2].sort();
            Some(r[(l - 1) / 2])
        }
    }
}

fn mode(v: &Vec<i32>) -> Option<i32> {
    if let 0 = v.len() {
        return None;
    }

    let mut h = HashMap::new();
    for e in v {
        let count = h.entry(*e).or_insert(0);
        *count += 1;
    }

    let (mut mkey, mut mval);
    mval = 0;
    mkey = 0;
    for (key, value) in h {
        (mkey, mval) = if value > mval {
            (key, value)
        } else {
            (mkey, mval)
        };
    }
    Some(mkey)
}
