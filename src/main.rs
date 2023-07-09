struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20,
    };
    let area = rect_area(rect);
    println!("area of rect is {}", area);
}
fn rect_area(rect: Rectangle) -> u32 {
    return rect.width * rect.height;
}
