fn main() {
    let mut s = String::from("hello world");

    let word = first_world(&s);

    // Because clear needs to truncate the String, it needs to get a mutable reference.
    s.clear();

    println!("the first word is: {word}");
}

fn first_world(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            // Whether the s is String or &String, always should return &s[..i].
            return &s[..i];
        }
    }

    &s[..]
}
