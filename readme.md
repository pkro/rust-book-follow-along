# rust book follow along

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [Installation](#installation)
- [basics](#basics)
- [compilation](#compilation)
- [Cargo (rust package manager)](#cargo-rust-package-manager)
- [Rust standard library and prelude](#rust-standard-library-and-prelude)
- [Dependencies / importing libraries](#dependencies--importing-libraries)
- [Programming a guessing game](#programming-a-guessing-game)
- [Concepts](#concepts)
  - [Variable / const declarations](#variable--const-declarations)
  - [Variable shadowing](#variable-shadowing)
  - [Data types](#data-types)
    - [Scalars: Integers](#scalars-integers)
    - [Scalars: Floating point, boolean, character](#scalars-floating-point-boolean-character)
  - [Compound types](#compound-types)
    - [Tuple](#tuple)
    - [Array](#array)
  - [Code blocks](#code-blocks)
  - [Functions](#functions)
  - [Comments](#comments)
  - [control flow](#control-flow)
    - [if / else / else if](#if--else--else-if)
    - [ternary](#ternary)
    - [loops (loop, while, for)](#loops-loop-while-for)
- [Ownership](#ownership)
  - [Exkurs: stack / heap](#exkurs-stack--heap)
  - [Ownership rules](#ownership-rules)
  - [Variable scope](#variable-scope)
  - [String / complex types / deep vs shallow copy / moving variables](#string--complex-types--deep-vs-shallow-copy--moving-variables)
  - [Ownership and functions](#ownership-and-functions)
  - [References and borrowing](#references-and-borrowing)
    - [Immutable references](#immutable-references)
    - [Mutable references](#mutable-references)
    - [Dangling references](#dangling-references)
  - [Slice type](#slice-type)
- [Structs](#structs)
  - [Structs with named fields](#structs-with-named-fields)
  - [Tuple Structs](#tuple-structs)
  - [Unit-like structs](#unit-like-structs)
  - [Display and Debug traits to display struct data](#display-and-debug-traits-to-display-struct-data)
  - [The `dbg!` macro](#the-dbg-macro)
  - [Methods and associated functions](#methods-and-associated-functions)
- [Enums and Pattern matching](#enums-and-pattern-matching)
  - [basic enums](#basic-enums)
  - [enums with types and enum methods](#enums-with-types-and-enum-methods)
  - [The Option Enum and its advantages over Null values](#the-option-enum-and-its-advantages-over-null-values)
  - [The `match` control flow construct](#the-match-control-flow-construct)
    - [Basics](#basics-1)
    - [Matching with `Option<T>`](#matching-with-optiont)
    - [Catch-all patterns and the `_` placeholder](#catch-all-patterns-and-the-_-placeholder)
    - [shorthand `if let`](#shorthand-if-let)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

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

A `crate` is a collection of rust source code files or binaries.

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
- declare constants with `const`; constants are inlined (replaced with their value) at compile time
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
- Numeric literals can be explicitly typed using suffixes, like `3u8` for an unsigned 8-bit integer (u8), specifying the exact data type.

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
- Conversions between numeric types must be explicit.


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

**A tuple without any values (`()`) is called *unit* and represents an empty value / empty return type.**

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
### Code blocks

Code blocks evaluate to the last expression in them. A semicolon changes an expression to a statement.

- `{ 3+5 }` evaluates to 8
- `{ 4+5; }` evaluates to unit (empty tuple)

### Functions

- main entry function name: `main`
- snake case by convention
- parameter types must be declared in the function signature
- return type (if not `()` (unit)) must be declared using `->`
- Can be declared inside any scope, , but it's generally best practice to define them at the module level (in the global scope of the module).
- **for the last expression of a function, it's idiomatic in Rust to omit return and the semicolon**
- return values can (or must?) be declared with an arrow
  - if the last line in a function is an expression, it is used as the return value
  - the expressions becomes a statement if it ends with a `;`, and thus isn't a return value anymore
  - best use normal `return` unless it's a one-liner ()

```rust
fn five() -> int32 {
  5 // ok, last expression in the code block
}

fn six() -> int32 {
  6; // error - 6 is a statement because of the ;
}

fn seven() -> int32 {
  return 7; // ok
}
```

### Comments

- All normal c-style comments work, `//`, `/* multiple lines */`

```rust
// idiomatic 
// multiline comment
```

### control flow

#### if / else / else if

- no paranthesis necessary for `if` expression (`if x < 5 {...}`)
- expression must evaluate to bool (no automatic conversion of e.g. a number to bool like in javascript)
- blocks of code associated with if conditions are called arms, like in `match`

#### ternary
 Is simply a terse if / else condition. The expression "returned" from both branches must be of the same type

- fine: `let is_even = if x % 2 == 0 { true } else { false };`
- wrong: `let is_odd = if x % 2 != 0 { "oh no" } else { return 42 };`

#### loops (loop, while, for)

- `loop` runs until `break` is called.
- `continue` works as expected
- loops can return values using `break`
- loops can be labeled using `'`; the label can be specified for `break` and `continue`; this is useful for loops within loops
- `while` as usual
- `for` loops use `in` to iterate over arrays etc.; they don't follow the common `(initializer; condition; increment / statement)` pattern  
  - can iterate over iterables

```rust
fn main() {
    let mut x = 0;
    loop {
        x = x + 1;
        println!("i run 10 times{x}");
        if x == 10 {
            break;
        }
    }

    x = 0;
    let y = loop {
        x+=1;

        if x > 20 {
            break x;
        }
    }; // if used as an expression, ";" is necessary

    println!("twentyone is {y}");

    x = 0;
    let z = 'outer_loop: loop {
        'inner_loop: loop {

            if x >= 5 {
                break 'inner_loop;
            }
            x = x + 1;
            println!("I print 5 times");
        }
        x = x + 1;
        if x > 15 {
            break 'outer_loop x; // just add return (if required) after the label
        }
        println!("And I print 10 times");
    };

    x = 0;
    while x < 10 {
        println!("Me too!");
        x+=1;
    }

    for el in [1,2,3,4,5] {
        println!("{el}"); // 1,2,3,4,5
    }

    // just to show how a range can be defined
    for el in (1..5).rev() { // tuple (5,4,3,2,1)
      println!("{el}");
    }

    loop {
        println!("I am an endless loop! Please kill me!");
    }
}
```

## Ownership

- set of rules that govern how rust manages memory
- rusts most unique feature
- enables memory-safe guarantees without need for a garbage collector

### Exkurs: stack / heap

- the **stack** stores values in the order it gets them and removes the values in the opposite order (last in, first out), like a stack of plates
  - faster than heap because the memory allocator doesn't have to search for empty space
  - values passed to a function go on the heap (including pointers to data on the heap)
- **heap**: 
  - >when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating)
  - a *pointer* to the heap can be pushed on the stack
  - slower because allocator must search for space big enough for the data, and for access a pointer must be used

### Ownership rules

- each value has an *owner*
- there can be only one owner at a time (meaning the owner can change?)
- when the owner goes out of scope, the value will be dropped

### Variable scope

- for primitive types (stored on the stack, size known at compile time) like usual, scope is in block (`{}`) in which they are defined

### String / complex types / deep vs shallow copy / moving variables

Complex string type that can be assigned during runtime.

A String is made of 3 parts: ptr (pointer to start of allocated memory on the heap), len (string length)and capacity (total memory received from the allocator).
This groups of data is stored on the stack.

```rust
fn main() {
    //simple datatypes on the stack
    {
        // side note: prefixing a variable with "_" lets the compiler know it's intentionally unused
        let _s = "hello"; // primitive str type, size know at compile time, hardcoded into executable
    } // s goes out of scope
    //println!("{}", s); // can't find value s in this scope

    {
        // complex string type
        // unknown size at compile time
        let mut s = String::from("hello"); // heap memory is requested from the allocator
        s.push_str(", world!");
        println!("{}", s);
    } // scope is over, s is no longer valid; memory is automatically returned
}
```

- Rust doesn't have a garbage collector, but when a variable goes out of scope, rust automatically calls a function called `drop`.
- Rust never makes deep copies of complex objects explicitly but must be done with `.clone()` methods (unless the variable type implements the `Copy` trait).
- again, for primitive types such as &str, all integer types, tuples if they _only_ contain primitive types, rust always does a deep copy
- types that implement the `Copy` trait are automatically copied, not moved
- if a type implements a `Copy` trait, it can't implement the `Drop` trait
- In Rust, the Copy trait is implemented by various simple and primitive types

```rust
fn main() {
    let x = 5;
    let y = x; // primitive type - value is copied on the stack

    let s1 = String::from("hello");
    // a new String object is created on the stack that points to the same memory location as s1
    // basically like a reference in javascript
    let s2 = s1; // in rust speak: s1 is moved to s2
    // from here, s1 is no longer valid and accessing it will lead to compile errors
    // this is done by rust so the allocator doesn't try to free memory twice
    // when both s1 and s2 go out of scope
    // println!("{s1}"); // compiler error
    println!("{s2}"); // fine

    let s3 = s2.clone(); // explicit deep copy, s2 stays valid
    println!("{s3}, {s2}"); // hello, hello

    print_me(s2);
    print_me(s2); // error! print_me took ownership so s2 is no longer valid!
}

fn print_me(s: String) {
  println!("{s}");
}

```

### Ownership and functions

- When passing a value to a function, they get either copied or moved according to the rules
- calling a function with an argument of a type that implements `Copy` means

Straight from the docs:

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
                                    // AND THIS IS THE SURPRISING PART

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

We can get the variable back by returning multiple values from a function:
 
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

But we can also pass the variable by reference to avoid this bullshit.

### References and borrowing

#### Immutable references

Passing args by reference:

```rust
fn main() {
    print_me(&s2);
    print_me(&s2); // no more error as we pass it by reference (see function signature with &)
}

fn print_me(s: &String) {
    s.push_str("this will not work"); // variables passed by reference can't be modified
    println!("{s}");
} // s goes out of scope, but it's just a reference to s2, so s2 is still fine
```

- Creating a reference is called *borrowing* in rust speak, with some difference to e.g. javascript.

Javascript:

```js
let obj = { value: 10 };
let another = obj;  // `another` is now a reference to the same object

another.value = 20; // Modifying the object via `another`
console.log(obj.value); // Reflects the change, outputs: 20
```

```rust
let mut s = String::from("hello");
let r = &s;     // Immutable borrow of `s`
// let r_mut = &mut s; // This line would cause a compile error

println!("s is {}", s); // OK: `s` can be accessed while it's immutably borrowed
// r_mut.push_str(", world"); // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
```
In Rust, the borrowing rules enforce at compile time that you cannot have mutable and immutable references to the same data in the same scope.

- There can me multiple **immutable** references to a value

#### Mutable references

Self explanatory:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Not self explanatory: if there's a mutable reference to a value, there can't be _any_ other references to the value.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let thisWillError = &mut s; // PROBLEM: an object with a mutable variable can't have any other references attached to it
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem, as r1 and r2 are not used anymore in the program / scope
    println!("{}", r3);
}
```

#### Dangling references

```rust

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s  // we (try to) return a reference to the string s
}       // but s goes out of scope here so it (it's memory) ceases to exist,
        // so the returned pointer would point to nothing

fn no_dangle() -> String {
    let s = String::from("hello");

    s // fine, whole string is returned (ownership is moved to calling scope)
}

```

### Slice type

- slices reference a contiguous sequence in a collection, similar to numpy views (no data is copied)
- range syntax: `[firstIndexInclusive..lastIndexExclusive]`, e.g. `String::from("hello")
- slices are a form of borrowing. When creating a slice from a collection, you're borrowing part of that collection.

```rust
fn main() {
    let s = String::from("Hello world");

    // create slices of the string
    let hello = &s[0..5]; // [firstPos..LastPosExclusive]
    let world = &s[6..11];

    println!("{}", &s[0..4]); // hell
    println!("{}", &s[6..]); // world
    println!("{}", &s[..]); // hello world
}
```
- **String slice ranges are not UTF-8 safe**; if a slice starts in the middle of a multibyte character, the program will crash
- String can be converted to a str using slices (but rust also converts a String to str automatically when needed)

```rust
fn main() {
    let s = String::from("Hello world");

    let res = first_word(&s);

    println!("{res}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

- slices work on most collection types, e.g. arrays.
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

## Structs

### Structs with named fields

Like a class or Object prototype in javascript, or a struct in C.

```rust
struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

fn main() {
  let mut user1 = User { // no "new" keyword to instantiate a value from a struct type
    sign_in_count: 1, // order is irrelevant
    active: true,
    username: String::from("abc"), // we want to change these later maybe,
    email: String::from("a@bc.de"), // so we don't use a str (which would be inlined at compile time)
  };

  // we can only re-assign if the whole struct instance is instantiated with "mut";
  // we can't set individual fields in a struct to "mut"
  user1.email = String::from("ab@c.de");

  // creating a User from another User
  let user2 = User {
    active: user1.active, // this is a primitive value that implements the copy trait so it's COPIED from user1
    username: user1.username,   // these are String value (complex object/value), so they are
    email: user1.email,         // MOVED to user2 and thus are NOT AVAILABLE IN user1 ANYMORE!
    sign_in_count: user1.sign_in_count
  };

  // shorthand for assigning from another User
  let user3 = User {
    email: String::from("another@example.com"),
    ..user2 // again, non-primitive values are MOVED to user3 so user2 isn't complete anymore
  };
}

// User is a type we can use as parameter or return types as well
fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    username,   // we can use shorthand notation like in javascript if a variable
    email,      // with the same name exists in the current scope (here from the parameters)
    sign_in_count: 1,
  }
}
```

### Tuple Structs

self-explanatory:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Unit-like structs

Structs without any data, used for example if we want to create methods on a struct but don't need any instance data.

```rust
struct AlwaysEqual;

fn main() {
  let subject = AlwaysEqual;
}
```

### Display and Debug traits to display struct data

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {}", rect1); // compile error because Rectangle doesn't have a Display trait,
                                    // meaning it doesn't know how to show itself as a string
                                    // similar to .toString() (js) or __rep__ (python)

    // println!("rect1 is {:?}", rect1);   // we say "use Debug trait as the string conversion method
                                        // doesn't exist either on our Rectangle struct
                                        // so it still errors



    #[derive(Debug)] // attribute to let rust add a debug trait to the struct
    struct RectangleWithDbg {
        width: u32,
        height: u32,
    }

    // structs can be defined in another scope, too
    let rect2 = RectangleWithDbg {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect2); // output: rect1 is RectangleWithDbg { width: 30, height: 50 }
    
    println!("rect1 is {:#?}", rect2); // note the #; multiline output;
    // output: 
    // rect1 is RectangleWithDbg {
    //     width: 30,
    //     height: 50,
    // }
}
```
### The `dbg!` macro

`dbg!(value)` takes ownership of a value, prints it out using the `Debug` trait and returns the value.

```rust
let rect3 = dbg!(rect2); // assigns and logs value to stdout using Debug trait
```

### Methods and associated functions

```rust
use std::cmp::min;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// methods are defined outside of the struct using impl
// (implementation block)
impl Rectangle {
    // like in python, self must be the first parameter if we want
    // to access the struct instance data
    // &self is shorthand for self: &Self
    // we use &self (immutable reference) because we don't want to take ownership / change anything
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // make self mutatable and add a parameter
    fn add_height(&mut self, units: u32) {
        self.height += 10;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // very rarely used, takes ownership of self
    // usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation
    fn convert_to_string(self) -> String {
        format!("{}, {}", self.height, self.width)
    }

    // associated functions that are not methods
    // basically static methods called with Type::function, e.g. String::from
    // these are distinguished by not having a self parameter
    fn square(side_length: u32) -> Self {    // returns an instance of the type it's called on;
                                            // -> Rectangle would be ok as well
        Self {
            width: side_length,
            height: side_length
        }
    }
}

// there can be multiple impl blocks for the same type
impl Rectangle {
    fn whatever(&self) {
        ()
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.area()); // 1500

    let mut rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    rect2.add_height(10);
    println!("{}", rect2.area()); // 1600

    let rectString = rect2.convert_to_string();
    // rect2.width; // compiler error - rect2 was taken ownership of and is no longer available
    println!("{}", rectString); // 60, 30

    let square = Rectangle::square(10);
    println!("{}", square.area()); // 100
}
```

## Enums and Pattern matching

### basic enums

```rust
// basic enum
enum IpAddrKind { // CamelCase for enums
    V4, // these are not existing types but enum variants
    V6, // that can be used as-is
}

// used as a param / type
fn route(ip_kind: IpAddrKind) {}

fn main() {
    // create instances of enum
    let four = IpAddrKind::V4;
    route(four);
    // use directly
    route(IpAddrKind::V6);
}
```

### enums with types and enum methods

```rust
// enum with types in variants
// the types of the variants can be anything: tuples, structs etc.
enum IpAddrKind {
    V4(u8, u8,u8,u8), // V4 is a tuple of 4 u8s
    V6(String)
}

// enums can have methods, too
impl IpAddrKind {
    fn show(&self) {
        // this is how we can access the variant of the current instance
        match self {
            IpAddrKind::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d), // destructure data
            IpAddrKind::V6(s) => println!("{}", s),
        }
    }
}
// used as a param / type
// note that here route takes ownership, so it vanishes from the calling scope after call
fn route(ip_kind: IpAddrKind) {}

fn main() {
    // create instances of enum
    let four = IpAddrKind::V4(127,0,0,1);
    route(four);
    // use directly
    route(IpAddrKind::V6(String::from("::1")));

    (IpAddrKind::V4(127,0,0,1)).show(); // 127.0.0.1
}
```

### The Option Enum and its advantages over Null values

- rust doesn't have a `null` value, but the `Option<T>` enum
- T = generic type, meaning the `Some` variant can hold any value

Option implementation from the standard library
```rust
enum Option<T> {
    None,
    Some(T),
}
```

_A value that we know can potentially become null must be of `Option<T>` type, no other value can become null(ish)_

**Using Option<T> and handling it with pattern matching or safe methods is a core part of Rust's approach to safety, ensuring that you deal with potential null or absent values explicitly.sing Option<T> and handling it with pattern matching or safe methods is a core part of Rust's approach to safety, ensuring that you deal with potential null or absent values explicitly.**

Using the `Option<T>` enum makes the compiler ensure we always handle the `None` (nullish) arm.

If we want to use the value of `Option<T>`, we must convert it to `T` first.

```rust
fn main() {
    let some_number = Some(5); // type is Option<i32>
    let some_char = Some('e'); // type is Option<char>

    let absent_number: Option<i32> = None; // basically null

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let i_error= x + y; // error

    // we MUST explicitely handle the possible None value
    let sum = match y {
        Some(num) => x + num,  // If y is Some(i8), add it to x
        None => x,            // If y is None, just use x
    };

    println!("{sum}"); // 10
}
```

### The `match` control flow construct

#### Basics

- `match` is one of the most important constructs in rust
- `match` ensures all possible values of a variable are covered
- arms can bind to the values of a variable so we can access its content (e.g. properties of a struct)
- the patterns are evaluated in the order they are defined, so it might be a good idea to put the 

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState) // an enum with a value, here it's another enum
}

fn main() {
    let a_penny = Coin::Penny;
    let a_quarter = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_cents(&a_penny));   // 1
    println!("{}", value_in_cents(&a_quarter)); // 25 (and prints "Quarter from Alaska")

}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,    // arms of the match expression
        Coin::Nickel => 5,   // return 1 if coin is a penny
        Coin::Dime => 10,
        Coin::Quarter(state) =>  { // we can use blocks, too
            println!("Quarter from {:?}", state); // side effect
            25                          // return 25
        }
    }
}
```

#### Matching with `Option<T>`

```rust
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // None and Some(x) are from the standard library 
    // and are included in the prelude
    // They return an Option value
    match x {
        None => None,               // without this line, there'd be an error as not all cases are covered
        Some(i) => Some(i+1)    // access the Some value
    }
}
```

#### Catch-all patterns and the `_` placeholder

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // all other values ("other", an arbitrary variable name, binds to the value)
        // other is bound to the actual value
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ matches any value but doesn't bind to the value
        // used to indicate to the compiler that the value will not be used
        _ => shrug_at_player(),
        // if we really don't do anything, we could also return a unit
        // _ => ()
        // or
        // _ => {}
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn shrug_at_player() {}
fn move_player(num_spaces: u8) {}
```

We can use function to modify the match value before evaluation:

```rust
fn random_number(value: &i32) -> u32 {
    5
}

fn main() {
    let secret_number = 42; // Example value

    match random_number(&secret_number) {
        4 => println!("Success!"),
        6 => println!("Failure!"),
        _ => {} // since rust can't determine random_number, it insists on a catch_all
    }
}
```

#### shorthand `if let`

- use if only one pattern / arm is of interest and the rest can be ignored
- takes a pattern and an expression separated by an equal sign
- "syntactic sugar" for `match` to make it more concise in commen if / else cases
- using if let with `Option` types, particularly with the `Some` variant, is one of the most common use cases for this construct
- Exhaustiveness Checking: Unlike `match`, which requires handling all possible cases, `if let` focuses on only one pattern, sacrificing the exhaustiveness check for conciseness. This is useful for simpler scenarios but requires caution in complex cases where unhandled variants might lead to logic errors.

```rust
fn main() {
    let config_max = Some(3u8); // numeric literal with type prefix
    // using match
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Using if let
    if let Some(max) = config_max { // this is basically assigning config_max to max
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Max connections are not configured");
    }
}
```