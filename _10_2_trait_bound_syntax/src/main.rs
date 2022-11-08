trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn trait_bound_syntax_notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("elonmusk"),
    };
    notify(&tweet);
    trait_bound_syntax_notify(&tweet);
}
