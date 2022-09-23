fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    // automatic referencing and dereferencing:
    // Rust automatically adds in &, &mut, or * so object matches the signature of the method.
    // `rect1.area()` equals `(&rect1).area()`.
    println!("The area of rect1 is {}.", rect1.area());
}

struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    // The &self is actually short for self: &Self.
    fn area(&self) -> u64 {
        self.width * self.height
    }
}