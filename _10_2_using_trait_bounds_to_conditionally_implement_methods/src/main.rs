trait Greeting {}

trait Halo {
    fn halo(&self);
}

// Implementations of a trait on any type that satisfies the trait bounds are called blanket
// implementations and are extensively used in the Rust standard library.
impl<T: Greeting> Halo for T {
    fn halo(&self) {
        println!("Halo!")
    }
}

struct Person {}

impl Greeting for Person {
}

fn main() {
    let person = Person {};
    person.halo()
}
