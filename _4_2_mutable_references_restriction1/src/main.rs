fn main() {
    // Mutable references have one big restriction: if you have a mutable reference to a value, you
    // can have no other references to that value.
    let mut s = String::from("hello");
    let r1 = &mut s;
    // The first mutable borrow is in r1 and must last until it’s used in the println!, but between
    // the creation of that mutable reference and its usage, we tried to create another mutable
    // reference in r2 that borrows the same data as r1.
    let r2 = &mut s;
    println!("{r1}, {r2}");

    // A data race is similar to a race condition and happens when these three behaviors occur:
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.
}
