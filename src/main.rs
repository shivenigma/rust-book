use std::io;

fn main() {
    println!("Enter the number you want to generate the Fibonacci number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to enter");
    let mut number: i32 = match  number.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Invalid number")
    };
    let mut fib: i32 = 1;
    let mut prev: i32 = 0;
    while number > 1 {
        let temp = prev;
        prev = fib;
        fib = temp + fib;
        number = number - 1;
    }
    println!("The fibonacci number is {fib}");
}
