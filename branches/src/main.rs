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

    for el in (1..5).rev() { // tuple (5,4,3,2,1)
        println!("{el}");
    }

    loop {
        println!("I am an endless loop! Please kill me!");
    }
}
