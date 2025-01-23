use std::io;

fn main() {
    loop {
        println!("Please enter un positive integer n. Enter 0 to exit the program");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("reading the number failed");
        let n: u32 = n
            .trim()
            .parse()
            .expect("failed to decode the number. Your input must be a positive or null integer");

        if n == 0 {
            break;
        }

        let result = fibonacci_term(n);
        println!("The {n}th Fibonacci term is {result}!");
    }
}

fn fibonacci_term(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    fibonacci_term(n - 1) + fibonacci_term(n - 2)
}
