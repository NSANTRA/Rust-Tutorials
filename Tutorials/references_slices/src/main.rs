fn main() {
    let mut s: String = String::from("Hello");

    // Immutable references
    let r1 = &s;
    let r2 = &s;

    println!("Value of r1 and r2 are {r1}, {r2}");
    
    // Mutable reference
    let r3 = &mut s;
    println!("Value of r3 are {r3}");

    let s: String = String::from("Hello, my name is Neel");

    let size = slice_reference(&s);

    println!("The first word of the given sentence is: {}", &s[0..size]);
}


// fn dangling_referenc() -> &String {
//     let s = String::from("Hello");
//     &s
// }        // Here, the scope of variable s is dropped and so does its value, therefore we cannot return a reference to s variable

fn slice_reference(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &c) in bytes.into_iter().enumerate() {
        if c == b' ' {
            return i
        }
    }

    s.len()
}