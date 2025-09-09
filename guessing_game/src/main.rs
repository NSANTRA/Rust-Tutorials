use std::cmp::{Ordering};
use std::io::{stdin};
use rand::{rng, Rng};

fn main() {
    let mut guess: String;
    let secret: i64 = rng().random_range(1..=100);
    
    loop {
        guess = String::new();
        
        // stdin().read_line(&mut x).expect("Failed to read the input");
        println!("Enter a value: ");
        match stdin().read_line(&mut guess) {
            Ok(_) => print!(""),
            Err(_) => println!("Failed to read the input"),
        };
        
        // let x: i64 = x.trim().parse().expect("Please type a number");
        let guess: i64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // println!("The secret number is {secret} and you guessed {x}");
        println!("You guessed {guess}");
        
        match guess.cmp(&secret) {
            Ordering::Less => {
                println!("Less than the secret");
            },
            
            Ordering::Equal => {
                println!("The secret number was {secret}");
                println!("You win!");
                break;
            },
    
            Ordering::Greater => {
                println!("Greater than the secret");
            },
        };
    };
}