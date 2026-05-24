fn main() {
    let height: u32 = 100;
    let width: u32 = 500;

    println!("Area of the reactangle is {}", area(width, height));
}

fn area(height: u32, width: u32) -> u32 {
    height * width
}
