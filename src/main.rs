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
    println!("The fibonacci number is {}", fib(number));
}

fn fib(num: i32)-> i32 {
    if num <= 0 {
        return 0;
    } else if num == 1 {
        return 1;
    } fib(num-1) + fib(num-2)
}
