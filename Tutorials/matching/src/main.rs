enum State {
    Alaska,
    NewYork,
}
enum Coin {
    Penny,
    Nickel,
    Dime(State),
    Quarter,
}

fn operation(x: Option<i64>) -> Option<i64> {
    match x {
        None => {
            None
        },
        Some(i) => {
            Some(i + 1)
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime(State::Alaska) => 10,
        Coin::Dime(State::NewYork) => 15,
        Coin::Quarter => 25,
    }
}

// fn main() {
    //     println!("{}", value_in_cents(Coin::Dime(State::NewYork)));
    
    //     let a = Some(5);
    //     let res = operation(a);
    // }
    
// Match functions are exhaustive, meaning every possibility has to be covered. In other words, None possibility should always be covered.
// Another example of match is by using catch-all statment:

fn add() {
    println!("Add")
}

fn sub() {
    println!("Sub")
}

fn other_func() {
    println!("Other")
}

fn main() {
    let a = 9;

    match a {
        3 => add(),
        7 => sub(),
        // other => other_func(),
        _ => other_func(),          // In case, we don't want to use values other than the ones we specified
    }
}