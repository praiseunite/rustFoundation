fn main() {

    // To use any arithmetic operation, we need to declare the data type of the variables involved in the operation.
    //both variables must be of the same data type. 
    //e.g if x is of type i32, y must also be of type i32.
    //if they are not of the same data type, the compiler will throw an error.

    println!("----------------------------------");
    println!("Addition");
    println!("----------------------------------");
    
    let x : i32 = 5;
    let y : i32 = 10;
    let sum : i32 = x + y;
    println!("Sum of {} and {} is {}", x, y, sum);
    
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("subtraction");
    println!("----------------------------------");

    let x : i32 = 10;
    let y : i32 = 5;
    let difference : i32 = x - y;
    println!("Difference of {} and {} is {}", x, y, difference);
    println!("----------------------------------");
    println!("  ");


    println!("----------------------------------");
    println!("Using different data types");
    println!("----------------------------------"); 

    //How to use different types of data types in arithmetic operations in Rust programming language. 
    //We can use the following code to perform arithmetic operations on different data types. 
    //The code below shows how to perform arithmetic operations on different data types in Rust programming language.

    let x : i32 = 10;
    let y : f64 = 5.0;
    let sum : f64 = x as f64 + y; //casting x to f64 to make it the same data type as y before performing the operation. 
    //Always go from the smaller data type to the larger data type.
    //casting is done by using the as keyword.
    println!("Sum of {} and {} is {}", x, y, sum);

    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Multiplication");
    println!("----------------------------------");

    let x : i32 = 10;
    let y : i32 = 5;
    let product : i32 = x * y;
    println!("Product of {} and {} is {}", x, y, product);
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Division");
    println!("----------------------------------");

}
