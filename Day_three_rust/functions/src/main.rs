use std::io;
fn main() {
    println!("Hello, world!");
    first_function();
    second_function("John");
    adding_numbers(10);

    println!("Enter your name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name) 
        .expect("Failed to read line");

    let name = name.trim();

    greetings(name);

    let result = returning_value(10);
    println!("The result is {}", result);

    //statements and expressions 
    // Statements are instructions that perform some action and do not return a value. 
    // Expressions are instructions that perform some action and return a value. 
    // In Rust, expressions do not end with a semicolon and statements end with a semicolon.
    // If an expression ends with a semicolon, it becomes a statement and does not return a value.
    let x = 10; // statement 
    let y = 20; // statement 
    let z = x + y; // expression 
    let a = { x + y }; // expression 
    let b = { x + y; }; // statement  // this will not return a value 
    let c = { x + y; z }; // statement 
    println!("The value of z is {}", z); // expression
    println!("The value of a is {}", a); // expression
    println!("The value of b is {:?}", b); // statement
}

// Functions
// print!("-------------------");
// Functions are defined using the fn keyword followed by the function name and the function body.
// The function body is enclosed in curly braces. The function body contains the code that the function will execute. 
// The function name is followed by parentheses. The parentheses can contain parameters that the function will accept. 
// The parameters are separated by commas. The parameters are defined using the parameter name followed by a colon and the parameter type.
// The function can return a value using the return keyword followed by the value to return. The return type is specified after the -> symbol.
// The return type is optional. If the function does not return a value, the return type is not specified. 
fn first_function() {
    println!("This is the first function");
}

fn second_function(name: &str) {
    println!("This is the second function {}", name);
}

fn adding_numbers(x: i32){
    println!("The number is {x}");
    println!("The number is {}", x);
}

fn returning_value(x: i32) -> i32 {
    x + 10
}

//Functions with user interraction
// Functions can accept input from the user using the std::io module.
// The std::io module provides the standard input/output functionality for reading and writing data.

fn greetings(name : &str) {
    println!("Hello, {}", name);
}

// Function Parameters 
// Function parameters are defined using the parameter name followed by a colon and the parameter type.
// The parameter type is optional. If the parameter type is not specified, the parameter type is inferred from the value passed to the function.
// The parameter type can be any valid Rust type. The parameter type can be a primitive type, a custom type, or a reference type.
// The parameter type can be a reference type. The reference type is specified using the & symbol followed by the parameter type.
// The reference type allows the function to access the value passed to the function without taking ownership of the value.
//example  fn add(x: i32, y: i32) -> i32 {
//example  fn add(x: i32, y: i32) -> i32 { x + y }  // this is a short form of the function