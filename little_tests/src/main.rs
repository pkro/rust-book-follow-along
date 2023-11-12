
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
