fn main() {
    let m = Message::Write(String::from("hello"));
    m.print();
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        dbg!(self);
    }
}