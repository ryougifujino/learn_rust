fn main() {
    let s = String::from("hello world");

    let word = first_word(&s[..]);
    let word = first_word(&s[..6]);
    let word = first_word(&s);  // &s equals &s[..]

    let s_literal = "hello world";
    // String literals *are* string slices.
    let word = first_word(s_literal);
    // `first_word` works on slices of string literals.
    let word = first_word(&s_literal[..]);
}

fn first_word(s: &str) -> &str{
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}