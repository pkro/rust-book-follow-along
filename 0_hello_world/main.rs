// every rust program starts with a main function
fn main() {
    // indentation doesn't (syntactically) matter
    // ""->string literals
    println!("Hello world");

    let x = 1;
    let x = "wowser";
    {
        let x = 55;
        println!(x) // 55
    }
    println!(x) // wowser
}