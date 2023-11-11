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