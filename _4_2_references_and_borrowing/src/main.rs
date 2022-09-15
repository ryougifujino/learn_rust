fn main() {
    let s = String::from("hello");

    let s_ref = &s;

    let s_ref2 = s_ref;

    println!("{}, {}", s_ref, s_ref2);

    let len = calculate_length(s_ref);

    println!("The length of '{s}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
