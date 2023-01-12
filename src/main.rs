mod calculator;
use calculator::{Calculator, Error};
use std::io;

/**
# Shunting Yard Calculator

 I followed along with this video: https://www.youtube.com/watch?v=KJwfZ06Z6og to learn rust.
 This is mostly his implementation, but I made a few changes that reduce repetition and test
 my understanding of `match`, `if let`, `while let`, `loop`, and `Some<T>` / `Option<T>`.

 ### There are a few problems with this implementation:
 1. It doesn't handle negatives or floating points.
 2. it doesn't actually handle parentheses correctly. e.g. 2 * (2 + 4) = 12, but this gives 8.
 */
fn main() -> Result<(), Error> {
    println!("\x1b[36mShunting Yard Calculator\x1b[0m");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("Input: {}", input);
                let tokens = Calculator::parse(input);
                if tokens.is_err() {
                    println!("\x1b[31mError:\x1b[0m {:?}", tokens.err().unwrap());
                    continue;
                    // exit(0)
                }
                let expr = Calculator::expression(tokens?);
                if let Some(v) = Calculator::evaluate(expr) {
                    println!("= {}", v);
                    continue;
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
