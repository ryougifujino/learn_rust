fn main() {
    let reference_to_notion = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}