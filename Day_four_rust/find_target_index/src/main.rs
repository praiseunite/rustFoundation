use std::io;

fn main() {

    println!("Please input the fibonacci number you want to find the index of: ");
    let mut mumber = String::new();
    io::stdin().read_line(&mut mumber).expect("Failed to read line");
    let number_to_find: u32 = mumber.trim().parse().expect("Please type a number!");

    // let number_to_find = 4181;
    match find_fibonacci_index(number_to_find) {
        Some(index) => println!("The Fibonacci number {} is at index {}.", number_to_find, index),
        None => println!("The number {} is not a Fibonacci number.", number_to_find),
    }
}

fn find_fibonacci_index(target: u32) -> Option<u32> {
    let mut a = 0;
    let mut b = 1;
    let mut index = 0;

    // Continue generating Fibonacci numbers until we reach or exceed the target
    while a <= target {
        if a == target {
            return Some(index); // Found the Fibonacci number
        }
        let next = a + b; // Calculate the next Fibonacci number
        a = b; // Move forward in the sequence
        b = next; // Update b to the next Fibonacci number
        index += 1; // Increment the index
    }
    
    None // Return None if the target is not a Fibonacci number
}
