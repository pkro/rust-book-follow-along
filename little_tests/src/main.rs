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