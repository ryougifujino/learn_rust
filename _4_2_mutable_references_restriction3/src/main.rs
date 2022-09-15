fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // Note that a reference’s scope starts from where it is introduced and continues through the
    // last time that reference is used.
    println!("{r1}, {r2}");

    let r3 = &mut s;
    println!("{r3}");
}
