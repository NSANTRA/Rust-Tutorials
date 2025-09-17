use std::vec;

fn main() {
    // We need to give the vector the annotations when declaring it.
    let mut v: Vec<i64> = Vec::new();
    
    // Updating vector using push (appending elements)
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    for &i in &v {
        print!("{} ", i);
    }
    println!();
    
    // This is another way of declaring a vector along with initializing values.
    let v2 = vec![1, 2, 3, 4];
    
    for &i in &v2 {
        print!("{} ", i);
    }
    println!();

    // The below method of retrieving elements from the vector can cause panic (error) in case we try to retrieve out of bounds index
    let third = &v2[2];
    println!("The third element of the vector is {}", third);
    
    // The below method of retrieving elements from the vector will not cause panic even if we try to access out of bounds index. It will handle the None value gracefully.
    let third = v2.get(2);
    match third {
        Some(third) => {
            println!("The third element of the vector is {}", third)
        }
        None => {
            println!("There is no third element")
        }
    }
}