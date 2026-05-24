fn main() {
    let rect1 = (100, 500);

    println!("Area of the reactangle is {}", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
