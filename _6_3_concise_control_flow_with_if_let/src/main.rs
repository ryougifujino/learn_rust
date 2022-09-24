fn main() {
    let coin = Coin::Quarter(&UsState::Alabama);

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => (),
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin<'a> {
    Penny,
    Nickel,
    Dime,
    Quarter(&'a UsState),
}
