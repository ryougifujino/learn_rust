fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    let r3 = &mut s;
    println!("{r1}, {r2}, {r3}");
}
