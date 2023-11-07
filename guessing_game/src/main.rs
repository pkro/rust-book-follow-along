// from the rust standard library
// std::io is not included in prelude, so we have to import it explicitely
use std::io;
// The Rng trait defines methods that random number generators implement,
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    // 1..=100 is an inclusive range expression
    let secret_number = rand::thread_rng().gen_range(1..=100); // random number 1-100 (inclusive)



    loop {
        println!("Please input your guess");
        // variables are declared using "let".
        // variables are immutable by default; to declare them as mutable, we add "mut"
        // ::new is an associated function implemented on the type String
        let mut guess = String::new(); // growable, UTF-8 encoded string

        // id we didn't import std::io, we could still use the function by
        // writing std::io::stdin
        io::stdin() // returns an instance of std::io::Stding type which represents a handle to stdin
            // pass in reference (&) to read_line and indicate its mutability with mut
            // read_lone returns a Result enum, a type that can be in one of multiple possible states ("variants")
            .read_line(&mut guess) // possible variants: Ok and Err
            // Values of the Result type have methods defined such as expect
            .expect("Failed to read line"); // Crash if result is Err
        // without expect, there would be a warning during compilation

        // convert guess to string for later comparison
        // the existing guess variable is shadowed so we can use the same name
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        // {} is a placeholder, similar to javascripts `you guessed: ${guess}`
        // We could have written println!("you guessed: {}", guess)
        println!("you guessed: {guess}, which was:");

        // match constructs ensure all cases / "arms" are handled
        match guess.cmp(&secret_number) { // cmp returns one of the arms patterns, e.g. Ordering:Less
            Ordering::Less => println!("too low"),       // "arms" of the match expression
            Ordering::Greater => println!("too high"),   // an arm consists of a pattern to match against
            Ordering::Equal => {
                println!("correct");
                break; // end the loop
            },       // and the code to run if it matches
        }

    }
}
