use std::{collections::HashMap};

fn main() {
    let mut map:HashMap<i32, String> = HashMap::new();

    // Appending to map using insert method
    map.insert(1, String::from("A"));
    map.insert(2, String::from("B"));
    map.insert(3, String::from("C"));

    println!("{map:?}");
    
    // Printing the values of hashmap using iteration
    for (key, value) in &map {
        println!("Key: {key}, Value: {value}");
    }
    
    // This is to validate whether the key is present or not. If present, no value change and if not present the specified value will be mapped.
    map.entry(4).or_insert(String::from("D"));

    println!("{map:?}");
}
