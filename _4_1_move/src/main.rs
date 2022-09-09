// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;    // s1 was invalid in order to avoid double free.

    println!("{s1}, world");
}
