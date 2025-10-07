// use rand::{rng, Rng};
use std::{io::stdin};

// fn create_vector(vec: &mut Vec<i64>) -> &mut Vec<i64> {
//     println!("Creating Vector...");
//     for _ in 0..5 {
//         vec.push(rng().random_range(1..100));
//     }
//     println!("Vector created");
    
//     vec
// }

fn display_vector(vec: &Vec<i64>) {
    println!("Vector: {:?}", vec);
}

fn search_by_index(vec: &Vec<i64>) {
    let mut index = String::new();
    stdin().read_line(&mut index).expect("Failed to read line");

    match index.trim().parse::<usize>() {
        Ok(index) => {
            match vec.get(index) {
                Some(value) => {
                    println!("Value at index {} is {}", index, value)
                }
                None => {
                    println!("Index out of bounds. Please enter an index between 0 and {}.", vec.len() - 1 );
                }
            }
        }
        Err(_) => {
            println!("Please enter a valid index (0-{}).", vec.len() - 1);
        }
    }
}

fn search_by_value(vec: &Vec<i64>) {
    let mut value = String::new();
    stdin().read_line(&mut value).expect("Failed to read line");

    match value.trim().parse::<i64>() {
        Ok(value) => {
            match vec.iter().position(|&x| x == value) {
                Some(index) => {
                    println!("Value {} found at index {}", value, index);
                },
                None => {
                    println!("Value {} not found in the vector", value);
                }
            }
        }
        Err(_) => {
            println!("Please enter a valid number.");
        }
    }
}

fn push_vector(vec: &mut Vec<i64>, length: usize) -> &mut Vec<i64> {
    if vec.len() >= length {
        println!("Vector is full. Cannot add more elements.");
        vec
    }

    else {
        let mut value = String::new();
        println!("Enter a number to add to the vector:");
        stdin().read_line(&mut value).expect("Failed to read line");
    
        match value.trim().parse::<i64>() {
            Ok(value) => {
                vec.push(value);
                println!("Value {} added to the vector", value);
            }
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
        
        vec
    }
}

fn pop_vector(vec: &mut Vec<i64>) -> &mut Vec<i64> {
    match vec.pop() {
        Some(value) => {
            println!("Value {} removed from the vector", value);
        }
        None => {
            println!("Vector is empty. Cannot pop elements.");
        }
    }
    vec
}

fn main() {
    let mut num: String = String::new();
    println!("Enter the maximum length of the vector:");
    stdin().read_line(&mut num).expect("Failed to read line");

    let length: usize = num.trim().parse().expect("Please type a number!");
    let mut vec: Vec<i64> = Vec::new();

    let mut choice: String = String::new();

    loop {
        println!("1. Add a number to the vector");
        println!("2. Show current vector");
        println!("3. Search number by index (0-4)");
        println!("4. Search number by value");
        println!("5. Remove last number");
        println!("6. Exit");
        println!("Enter your choice:");

        choice.clear();
        stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                // vec.clear();
                push_vector(&mut vec, length);
            }
            "2" => {
                display_vector(&vec);
            }
            "3" => {
                search_by_index(&vec);
            }
            "4" => {
                search_by_value(&vec);
            }
            "5" => {
                pop_vector(&mut vec);
            }
            "6" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please enter 1 to add a number.");
            }
        }
        println!()
    }
}