// Enum is a type of data type that is used store all possible variants of a particular value. Like directions has 4 variants, north, south, east, and west.

// Below is a simple declaration of enum custom data type
// enum Directions {
//     North,
//     South,
//     East,
//     West
// }

// We can also store values with each enum variant, similar to a structure. We can also write functions using implementation.
enum Message {
    Quit,
    Move {x: i64, y: i64},
    ChangeColor(String)
}

impl Message {
    fn walk(self: &Self) {
        println!("Moving");
    }
}

fn main() {
    let moveto: Message = Message::Move { x: 20, y: 5 };
    moveto.walk();

    // Rust does not have a standalone Null value, it is to be used with Option enum.
    // enum Option<T> {
    //      None,
    //      Some(T)
    // }

    // In the above code snippet is the implementattion of Option enum, with T being the generic data type that can hold any data type.
    let somenumber: Option<i32> = Some(5);
    let nonenumber: Option<i32> = None;
}