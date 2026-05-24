struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Area of the reactangle is {}", area(rect1));
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
