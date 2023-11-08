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