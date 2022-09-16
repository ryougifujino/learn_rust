fn main() {
    let pos = first_word(&String::from("Hello, world!"));
    println!("{pos}");
}

fn first_word(s: &String) -> usize {
    for (i, item) in s.as_bytes().iter().enumerate() {
        if item == &b' ' {  // b' ' is byte literal syntax
            return i;
        }
    }
    
    s.len()
}