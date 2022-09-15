fn main() {
    let pos = first_world(&String::from("Hello, world!"));
    println!("{pos}");
}

fn first_world(s: &String) -> usize {
    for (i, item) in s.as_bytes().iter().enumerate() {
        if item == &b' ' {  // b' ' is byte literal syntax
            return i;
        }
    }
    
    s.len()
}