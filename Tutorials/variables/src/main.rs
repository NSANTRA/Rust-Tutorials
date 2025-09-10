fn main() {
    // Immutable
    let a = 5;
    println!("The variable a is immutable. By default immutable, when using 'let' keyword. Value: {a}");
    
    // Mutable
    let mut b = 10;
    println!("The variable b is mutable. Can be made mutable using 'mut' keyword with 'let' keyword. Value: {b}");

    {
        println!("The value of b in inner scope is {}", (b + 5));
    }

    b = 20;
    println!("The variable b is mutable. Value: {b}");
    
    const GRAVITY: f64 = 9.6;
    // The const variable name should be UpperCase (warning)
    // Should always mention the data type
    println!("The variable c is always immutable. Uses 'const' keyword. Value: {GRAVITY}");

    /*
        There are 2 types of data types:
        1. Scalar: The scalar are the primitive data types - Integer, Float, Boolean, Character
        2. Compound: The data type that group multiple values into one single type - Array, Tuple
    */

    // Tuple
    let tup: (f64, i64, &str) = (9.6, 10, "Gravity");

    // Accessing individual elements
    // Example 1
    let (x, y, z) = tup;
    println!("The value of x: {x}");
    println!("The value of y: {y}");
    println!("The value of z: {z}");
    
    // Example 2
    // In this we cannot use format string - ("{var}")
    // We have to use - ("{}", var)
    println!("The value of 1st element of tup: {}", tup.0);
    println!("The value of 2nd element of tup: {}", tup.1);
    println!("The value of 3rd element of tup: {}", tup.2);

    // Array
    // Similar to tuple but unlike tuple, all elements of an array should be same.
    // Below are the 2 ways of declaring arrays
    // let a = [1, 2, 3, 4, 5];
    let a: [i64; 5] = [1, 2, 3, 4, 5];

    // Accessing array elements
    // Similar to tuple, we cannot use format strings when accessing array elements
    println!("1st element: {}", a[0]);
    println!("2nd element: {}", a[1]);
    println!("3rd element: {}", a[2]);
    println!("4th element: {}", a[3]);
    println!("5th element: {}", a[4]);
}