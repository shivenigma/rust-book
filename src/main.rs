struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20,
    };
    println!("area of rect is {}", rect_area(&rect));
}
fn rect_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
