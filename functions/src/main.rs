use std::{io::stdin, process::exit};

// This is a user-defined function that does not take any parameter, no return statement
fn no_param_func_no_return() {
    println!("User defined Function");
}

// This is a user-defined function that takes 2 arguments as parameters, no return statement
fn param_func_no_return(x: i64, s: &String) {
    println!("User entered: {x}, {s}");
}

// This is a user-defined function that does not take any argument, but returns 2 values as tuple
fn no_param_func_return() -> (i64, String) {
    let mut x: String = String::new();
    let mut s: String = String::new();
    
    println!("Enter Age: ");
    stdin().read_line(&mut x).expect("Failed to read line");
    
    let x: i64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid integer");
            exit(1);
        },
    };
    
    println!("Enter Name: ");
    stdin().read_line(&mut s).expect("Failed to read line");
    
    (x, s)
}

// This is a user-defined function that takes 2 arguments and returns them as tuple
fn param_return(x: i64, s: &String) -> (i64, &String) {
    (x, s)
}

fn main() {
    let mut x: String = String::new();
    let mut s: String = String::new();
    
    println!("Enter Age: ");
    stdin().read_line(&mut x).expect("Failed to read line");
    
    let x: i64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid integer");
            return;
        },
    };
    
    println!("Enter Name: ");
    stdin().read_line(&mut s).expect("Failed to read line");
    
    
    no_param_func_no_return();
    param_func_no_return(x, &s);
    
    {
        let (a, b) = no_param_func_return();
        println!("User entered: {a}, {b}");
    }
    
    {
        let (a, b) = param_return(x, &s);
        println!("User entered: {a}, {b}");
    }
}