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

- `cargo new projectname`: creates a new cargo project
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

## Concepts

### Variable / const declarations

- declare immutable ariables with `let`
- declare mutable variables with `let mut`
- declare constants with `const`; 
  - unlike `let`, a constant's type must always be declared
  - constant expressions are *always* evaluated at compile time [and as such are more limited in what can be assigned](https://doc.rust-lang.org/stable/reference/const_eval.html)
  - `mut` is obviously not allowed with constants
  - constants can be declared in any scope, including the global scope (outside functions)
  - naming convention: ALL_UPPERCASE_WITH_UNDERSCORES

### Variable shadowing

Variables can be shadowed (redeclared) in sub-scopes (like in javascript) and even in the same scope (the variable then never returns to the originally assigned value)

```rust
fn main() {
  let x = 1;
  println!("{x}"); // 1 - just so the compiler doesn't complain that x isn't used
  let x = "wowser";
  {
    let x = 55;
    println!("{x}") // 55
  }
  println!("{x}") // wowser
}
```

The variable name "x" is re-used here. The variable is still immutable as "x = 5" would produce a compiler error and only *explicitely* re-declaring the variable using `let` shadows the variable.

### Data types

- rust is statically typed, meaning all variable's data types must be known at compile time.
- rust can usually infer the type from the value; if it can't, it will complain

#### Scalars: Integers

```text
Length	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32 <- i32 is default
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize <- depends on architecture (usually 64 or 32 bit)
```

Possible number literal assignment syntax:

```text
Number literals	Example
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'
```
Overflow handling:

- In **debug mode**, rust checks for integer overflow during runtime and panics 
- In **release mode**, Rust doesn't check for possible integer overflows and in case of overflow wraps around (e.g. an `u8` with value 250 will become 5 if 10 is added to it) 

The standard library provides `wrapping_`, `overflowing_`, `checked_` and `saturating_` such as `wrapping_add` methods to explicitly handle behavior during runtime

- Wrap in all modes with the `wrapping_*` methods, such as wrapping_add.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*` methods (e.g. a variable with value 255 stays 255 when something is added to it)

#### Scalars: Floating point, boolean, character

- Floating point: `f32` and `f64`, both signed.
- boolean: `bool`
- character: `char`; must be declared with 's'ingle quotes; Unicode '😻'

### Compound types

Compound types can group multiple values in one type.

#### Tuple

Self explaining code:

```rust
fn main() {
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let second_value = tup.1; // 6.4
  let (x, y, z) = tup; // destructuring like in javascript except () instead of {}
}
```

#### Array

- All values in an array must have the same type
- arrays have a fixed length (if a variable length is needed, use `vector` from the stl)
- allocated in the stack
- Rust will panic during runtime if an out-of-bounds index is accessed

```rust
fn main() {
  let a = [1, 2, 3, 4, 5];
  let b: [i32; 5] = [1, 2, 3, 4, 5]; // explicitly state type and length
  let c = [3; 5]; // creates an array of length 5 with all values being 3 ([3, 3, 3, 3, 3])
  
  let a_second = a[1]; 
}
```

### Functions
