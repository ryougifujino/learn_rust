fn main() {
    // The :: syntax is used for both associated functions and namespaces created by modules.
    println!("{:#?}", Rectangle::square(50));
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn square(size: u64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}