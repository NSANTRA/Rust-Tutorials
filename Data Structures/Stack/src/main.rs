use std::{io::stdin};

fn push_stack(vec: &mut Vec<i64>, length: usize) {
    match vec.len() >= length {
        true => {
            println!("Overflow");
        },
        false => {
            let mut num: String = String::new();
            stdin().read_line(&mut num).expect("Failed to read the line");

            match num.trim().parse::<i64>() {
                Ok(num) => {
                    vec.push(num);
                    println!("Value {} added to the vector", num);
                }
                Err(_) => {
                    println!("Please enter a number");
                }
            }
        }
    }
}

fn pop_stack(vec: &mut Vec<i64>) {
    match vec.pop() {
        Some(value) => {
            println!("The popped value is {}", value);
        }
        None => {
            println!("Underflow");
        }
    }
}

fn peek(vec: &mut Vec<i64>) {
    match vec.is_empty() {
        true => {
            println!("Stack is empty!");
        }
        false => {
            println!("The topmost value of the stack is {}", vec[vec.len() - 1])
        }
    }
}

fn display(vec: &Vec<i64>) {
    println!("Vector is {:?}", vec)
}

fn main() {
    let mut num: String = String::new();
    let mut vec: Vec<i64> = Vec::new();
    
    println!("Enter the maximum length of the stack:");
    stdin().read_line(&mut num).expect("Failed to read the line");

    let length: usize = num.trim().parse().expect("Please enter a number");

    let mut choice: String = String::new();

    loop {
        stdin().read_line(&mut choice).expect("Failed to read the line");

        match choice.trim() {
            "1" => {
                push_stack(&mut vec, length);
            }
            _ => {
                println!("Please enter a valid option.");
            }
        }
    }
}
