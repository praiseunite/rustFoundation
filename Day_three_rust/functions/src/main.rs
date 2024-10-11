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

    // let result = returning_value(10);
    // println!("The result is {}", result);

    //statements and expressions 
    // Statements are instructions that perform some action and do not return a value. 
    // Expressions are instructions that perform some action and return a value. 
    // In Rust, expressions do not end with a semicolon and statements end with a semicolon.
    // If an expression ends with a semicolon, it becomes a statement and does not return a value.
    // let x = 10; // statement 
    // let y = 20; // statement 
    // let z = x + y; // expression 
    // let a = { x + y }; // expression 
    // let b = { x + y; }; // statement  // this will not return a value 
    // let c = { x + y; z }; // statement 
    // println!("The value of z is {}", z); // expression
    // println!("The value of a is {}", a); // expression
    // println!("The value of b is {:?}", b); // statement

    let result = fibonacci(10);
    println!("The result is {}", result);
}

//Fibonacci sequence  0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55  // 0 + 1 = 1, 1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8, 5 + 8 = 13, 8 + 13 = 21, 13 + 21 = 34, 21 + 34 = 55
// The Fibonacci sequence is a series of numbers in which each number is the sum of the two preceding numbers.
// The first two numbers in the Fibonacci sequence are 0 and 1.
// The Fibonacci sequence is defined by the recurrence relation: F(n) = F(n-1) + F(n-2)
// The Fibonacci sequence is a recursive sequence because it depends on the previous two numbers in the sequence.
// The Fibonacci sequence is used in mathematics, computer science, and other fields to model growth and other phenomena.
// The Fibonacci sequence is named after the Italian mathematician Leonardo Fibonacci who introduced the sequence to the Western world in his book Liber Abaci in 1202.
// The Fibonacci sequence is a classic example of a recursive sequence in mathematics.
// The Fibonacci sequence is used in computer science to demonstrate recursion and other concepts.

// Fibonacci sequence using recursion  
// The Fibonacci sequence can be generated using recursion.
// Recursion is a programming technique in which a function calls itself to solve a problem.
// Recursion is used to solve problems that can be broken down into smaller subproblems.
// Recursion is used to solve problems that have a recursive structure.
// example of recursion in rust

fn fibonacci(n: u32) -> u32 { 
    if n == 0 { 
        return 0; 
    }
    if n == 1 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// The fibonacci function takes an unsigned 32-bit integer n as a parameter and returns an unsigned 32-bit integer.

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