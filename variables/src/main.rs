fn main() {
    // Immutable
    let a = 5;
    println!("The variable a is immutable. By default immutable, when using 'let' keyword. Value: {a}");
    
    // Mutable
    let mut b = 10;
    println!("The variable b is mutable. Can be made mutable using 'mut' keyword with 'let' keyword. Value: {b}");

    b = 20
    println!("The variable b is mutable. Value: {b}");
    
    const c = 9.6;
    println!("The variable c is always immutable. Uses 'const' keyword. Value: {c}");
}