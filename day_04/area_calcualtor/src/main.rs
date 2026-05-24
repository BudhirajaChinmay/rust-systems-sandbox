#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 30;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 45,
    };

    dbg!(&rect1);
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
