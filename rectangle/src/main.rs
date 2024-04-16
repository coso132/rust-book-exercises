#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, r: &Rectangle) -> bool {
        (self.width >= r.width) && (self.height >= r.height)
    }
}

fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };
    let r1 = Rectangle {
        width: 30,
        height: 20,
    };
    let r2 = Rectangle {
        width: 1,
        height: 400,
    };
    dbg!(&r);
    dbg!(r.area());
    dbg!(r.can_hold(&r1));
    dbg!(r.can_hold(&r2));
}
