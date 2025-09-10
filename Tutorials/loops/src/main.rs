use std::{io::stdin, io::stdout, io::Write};

// Pyramid code using loop
fn pyramid_loop(x: i64) {
    let mut i: i64 = 1;

    'outer: loop {
        if i > x {
            break 'outer;
        }

        else {
            let mut j: i64 = 0;
    
            'inner: loop {
                if j == i {
                    break 'inner;
                }

                else {
                    print!("*");
                    j += 1;
                }
            }

            i += 1;
            println!("");
        }
    }
}

fn pyramid_while(x: i64) {
    let mut i: i64 = 1;

    while i != x + 1 {
        let mut j: i64 = 0;

        while j != i {
            print!("*");
            j += 1
        }

        println!("");
        i += 1
    }
}

fn pyramid_for(x: i64) {
    for i in 1..(x + 1) {
        for _ in 0..i {
            print!("*");
        }
        println!("");
    }
}

fn main() {
    let mut input: String = String::new();

    print!("Enter the number of rows: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read the line");

    let input: i64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("Enter a valid and positive integer");
            1   // Exit code (1: Failure)
        }
    };

    pyramid_loop(input);
    pyramid_while(input);
    pyramid_for(input);
}