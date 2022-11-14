fn main() {
    let string1 = String::from("the longer string");
    let result;
    {
        let string2 = String::from("the shorter");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("{}", result);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
