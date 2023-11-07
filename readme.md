# rust book follow along

## Installation

https://doc.rust-lang.org/stable/book/ch01-01-installation.html

## basics

By convention, the main entry point is in a file called `main.rs`.

`main.rs`
```rust
// every rust program starts with a "main" function
fn main() {
    // indentation doesn't (syntactically) matter, convention is 4 spaces
    // ""->string literals
    println!("Hello world"); // ! (shebang) means println! is a macro instead of a normal function
}
```

## compilation

Simple: `rustc main.rs`

## Cargo (rust package manager)

Usually, cargo is used to compile more complex applications.

- `Cargo.toml`: project configuration
- `cargo build`: compiles the project
- `cargo build --release`: compiles with optimizations
- `cargo run`: compiles + runs
- `cargo check`: checks the code without creating an executable

## Rust standard library and prelude

https://doc.rust-lang.org/stable/std/prelude/index.html

Prelude: parts of the standard library that rust automatically imports into every program, e.g. std::string

## Dependencies / importing libraries

A `crate` is a collection of rust source code files.

Example: `rand` is a *library crate*.

Cargo.toml:

```toml
[dependencies]
# import rand crate
rand = "0.8.5" # semantic versioning
```

The crates get automatically downloaded and imported by `cargo build`.

- `cargo build` creates a Cargo.lock file with fixed versions for reproducible builds, just like package.lock.json in node.
- `cargo update` updates the dependencies according to their semantic versioning and creates a new Cargo.lock file.
- `cargo doc --open` creates documentation *for the libraries used in the project* and opens it in the browser

## Programming a guessing game

```rust
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

```

