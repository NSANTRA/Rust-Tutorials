fn main() {
    let mut s: String = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("Value of r1 and r2 are {r1}, {r2}");
    
    let mut r3 = &mut s;
    println!("Value of r3 are {r3}");
}