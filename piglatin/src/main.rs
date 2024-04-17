use std::io;

fn main() {
    loop {
        let mut s = "".to_owned();
        match io::stdin().read_line(&mut s) {
            Err(_) => continue,
            _ => (),
        };

        match pig_latin(s.trim()) {
            None => continue,
            Some(y) => println!("{}", y),
        }
        //dbg!("{}", pig_latin(s.trim().to_string()));
    }
}

fn pig_latin(s: &str) -> Option<String> {
    let mut c = s.chars();
    match c.clone().enumerate().next() {
        None => return None,
        Some((_, ch)) => Some(match &ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                format!("{}-hay", c.collect::<String>())
            }
            'a'..='z' => format!(
                "{}-{}ay",
                {
                    c.next();
                    c.as_str()
                },
                ch
            ),
            _ => return None,
        }),
    }
}
