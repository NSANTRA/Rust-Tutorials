use std::io::stdin;

fn enqueue(vec: &mut Vec<i64>, length: usize) {
    match vec.len() >= length {
        true => {
            println!("Overflow");
        }
        false => {
            let mut value: String = String::new();
            print!("Enter a number:");
            stdin()
                .read_line(&mut value)
                .expect("Failed to read the line");

            match value.trim().parse::<i64>() {
                Ok(value) => {
                    vec.push(value);
                    println!("Enqueued value {} to the queue", value);
                }
                Err(_) => {
                    println!("Please enter a valid number");
                }
            }
            value.clear();
        }
    }
}

fn dequeue(vec: &mut Vec<i64>) {
    match vec.get(0) {
        Some(_) => {
            println!("Value {} dequeued from queue", vec.remove(0));
        }
        None => {
            println!("Underflow");
        }
    }
}

fn peek(vec: &Vec<i64>) {
    match vec.is_empty() {
        true => {
            println!("Queue is empty");
        }
        false => {
            println!("The front-most value in the queue is {}", vec[0]);
        }
    }
}

fn display(vec: &Vec<i64>) {
    match vec.is_empty() {
        true => {
            println!("Queue is empty");
        }
        false => {
            println!("The queue is {:?}", vec);
        }
    }
}

fn main() {
    let mut num: String = String::new();
    let mut vec: Vec<i64> = Vec::new();

    println!("Enter the maximum length of the queue:");
    stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");

    let length: usize = num.trim().parse().expect("Please enter a number");

    let mut choice: String = String::new();

    loop {
        println!("1. Enqueue");
        println!("2. Dequeue");
        println!("3. Peek");
        println!("4. Display");
        println!("5. Exit");

        stdin()
            .read_line(&mut choice)
            .expect("Failed to read the line");

        match choice.trim() {
            "1" => {
                enqueue(&mut vec, length);
            }
            "2" => {
                dequeue(&mut vec);
            }
            "3" => {
                peek(&mut vec);
            }
            "4" => {
                display(&vec);
            }
            "5" => {
                println!("Exitting...");
                break;
            }
            _ => {
                println!("Please enter a valid option.");
            }
        }
        println!("");
        choice.clear();
    }
}
