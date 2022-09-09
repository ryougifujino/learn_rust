fn main() {
    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);
    takes_ownership(s2); // s2 was no longer valid

    println!("{s2}");
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
