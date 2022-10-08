mod garden;

fn main() {
    let plant = crate::garden::vegetables::Asparagus {};
    println!("I am growing {:?}!", plant);
}