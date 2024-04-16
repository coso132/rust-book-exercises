fn main() {
    for a in 0..10 {
        print!("{}\n", fib(a));
    }
    //song();
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

/*fn song() {
    let song = [
        "On the twelfth day of Christmas,",
        "my true love gave to me",
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nin ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
        "And a partridge in a pear tree!",
    ];

    let mut i = 0;
    for _ in song {
        i += 1;
        for i in 0..i {
            println!("{}", song[i]);
        }
        println!("");
    }
}*/
