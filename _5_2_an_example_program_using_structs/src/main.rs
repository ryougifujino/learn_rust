fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 100,
    };

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
