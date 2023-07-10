#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 21,
    };
    println!("The rect is {:#?}", rect);
    println!("area of rect is {}", rect.area());
    println!("Rectangle fit {}", rect.can_hold(&rect2));
    println!("Rectangle fit {}", rect2.can_hold(&rect));
}
