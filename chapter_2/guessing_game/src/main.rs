
// use input/output library
use std::io;
// use rand from rand crate we installed
use rand::Rng;

use std::cmp::Ordering;


fn main() {

    // generate secret number
    // 101 is upper bound (exclusive)
    let secret_num = rand::thread_rng().gen_range(1..10);

    loop {

        // print macros
        println!("Guess number:");
        println!("input guess.");

        // mutable variable with associated function new of String type
        let mut guess = String::new();

        // pass reference to variable into function
        // handle failure with expect / result type
        // Result is enum
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // dont break following program by catching non-numbers

        // convert string to number
        // guess is shadow variable
        // let guess: u32 = guess.trim().parse().expect("not a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {}", guess);

        // use match expression to decide what to do based on value returned
        // order of "arms" in match is important (?)
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correct guess");
                break;
            }
        }

    }



}
