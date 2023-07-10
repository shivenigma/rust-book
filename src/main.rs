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
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
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
    let square = Rectangle::square(4);
    println!("The rect is {:#?}", square);
    println!("area of rect is {}", rect.area());
    println!("Rectangle fit {}", rect.can_hold(&rect2));
    println!("Rectangle fit {}", rect2.can_hold(&square));
}
