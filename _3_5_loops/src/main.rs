fn main() {
    // loop as expression
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // for with range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
