use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");

    // `rand::thread_rng()` gives us the "random number generator" we're going to use.
    // we call the `.gen_range()` method on the random number generator. This method
    // is defined by the `Rng` trait that we brought into scope at the top.
    // The range expression we're using here takes the start..=end format. `1..=100` is inclusive
    // to the upper and lower bounds, which essentially gives us a random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // `String` is Rust's textual string type.
        let mut guess = String::new();

        // `stdin()` is a handle to the standard input of the console.
        // `read_line()` will populate the guess variable with the user input in the console.
        // `expect()` will cause the program to crash if `read_line()` fails with an error message.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // We "shadow" (reuse) the guess variable name so we don't have to create a new variable with
        // a different name (e.g., like guess_str).
        // `: u32` annotates the type as an unsigned 32-bit integer.
        // trim() removes any whitespace and/or newlines and carriage returns in the string.
        // parse() converts string types to another type, in this case it will be of u32 since that's
        // what we set in the annotation.
        let guess: u32 = match guess.trim().parse() {
            // `parse()` returns a Result enum, which will either be Ok or Err.
            Ok(num) => num, // num is a success value which will populate the guess variable.

            // _ is a catch all that will be ignored and continue will go to  the next iteration
            // of the loop
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
