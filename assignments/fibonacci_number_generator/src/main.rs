use std::io;

fn generate_fibonacci(mut n: u32) -> u64 {
    let mut prev = 0;
    let mut curr = 1;

    while n > 0 {
        let temp = prev;
        prev = curr;
        curr = curr + temp;
        n = n - 1;
    }
    return prev;
}

fn main() {
    println!("Enter a number n to generate nth Fibonacci number: ");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line.");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not a vaild number."),
    };
    println!("{}", generate_fibonacci(n));
}
