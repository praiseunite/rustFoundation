//to be able to receive input from the user, we need to use the standard library
use std::io;

fn main() {
    println!("What is your name?"); // print a message to the user
    let mut input = String::new(); // create a mutable variable to store the user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input"); // read the user input and store it in the variable
    println!("Hello, {}", input); // print the user input

    println!("-------------------------------");

    println!("What is your age?"); // print a message to the user
    let mut my_imput = String::new(); // create a mutable variable to store the user input
    io::stdin()
        .read_line(&mut my_imput)
        .expect("Failed to read input"); // read the user input and store it in the variable
    let int : u32 = my_imput
        .trim()
        .parse()
        .expect("input is not a valid number"); // parse the user input to an unsigned 32bit integer
    println!("you are: {}", int); // print the integer value
}
