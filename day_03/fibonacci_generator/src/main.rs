use std::io;
fn main() {
    println!("Welcome to the Fibonacci generator!");

    println!("Input n, nth fibonacci number you want to generate!");

    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let n: u32 = match input_string.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please enter a number!"),
    };

    println!("nth Fibonacci number: {}", generate_n_fibonacci(n));
}

fn generate_n_fibonacci(n: u32) -> u128 {
    if n == 1 {
        return 0
    } else if n == 2 {
        return 1
    }

    let mut first: u128 = 0;
    let mut second: u128 = 1;

    for _ in 2..n {
        let result = first + second;
        first = second;
        second = result;
    }

    second
}
