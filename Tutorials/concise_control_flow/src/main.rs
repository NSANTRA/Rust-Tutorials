use std::fmt::format;

#[derive(Debug)]
enum State {
    Alaska,
    Albania,
}

impl State {
    fn existed_in(self: &Self, year: i64) -> bool {
        match self {
            Self::Alaska => {
                year >= 1800
            },
            Self::Albania => {
                year >= 1900
            }
        }
    }
}

enum Coin {
    Quarter(State),
}

fn func(coin: Coin, year: i64) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?}Made in the year {}", year))
        }

        else {
            Some(format!("Else part"))
        }
    }

    else {
        None
    }
}

fn main() {
    func(Coin::Quarter(State::Alaska), 1845);
}