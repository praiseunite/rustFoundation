fn main() {

    // To use any arithmetic operation, we need to declare the data type of the variables involved in the operation.
    //both variables must be of the same data type. 
    //e.g if x is of type i32, y must also be of type i32.
    //if they are not of the same data type, the compiler will throw an error.

    println!("----------------------------------");
    println!("Addition");
    println!("-  --  --  --");
    
    let x : i32 = 5;
    let y : i32 = 10;
    let sum : i32 = x + y; //sum is the result of addition operation.
    println!("Sum of {} and {} is {}", x, y, sum);
    
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("subtraction");
    println!("-  --  --  --");

    let x : i32 = 10;
    let y : i32 = 5;
    let difference : i32 = x - y; //difference is the result of subtraction operation. 
    println!("Difference of {} and {} is {}", x, y, difference);
    println!("----------------------------------");
    println!("  ");


    println!("----------------------------------");
    println!("Using different data types");
    println!("-  --  --  --");

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
    println!("-  --  --  --");

    let x : i32 = 10;
    let y : i32 = 5;
    let product : i32 = x * y; //product is the result of multiplication operation.
    println!("Product of {} and {} is {}", x, y, product);
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Division");
    println!("-  --  --  --");

    let x : i32 = 10; 
    let y : i32 = 5; 
    let quotient : i32 = x / y; //quotient is the result of division operation.
    println!("Quotient of {} and {} is {}", x, y, quotient); 
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------"); 
    println!("Modulus"); 
    println!("-  --  --  --"); 

    let x : i32 = 10; 
    let y : i32 = 5; 
    let remainder : i32 = x % y; //remainder is the result of modulus operation.
    println!("Remainder of {} and {} is {}", x, y, remainder); 
    println!("----------------------------------"); 
    println!("  "); 

    println!("----------------------------------");
    println!("Increment and Decrement"); 
    println!("-  --  --  --"); 

    let mut x : i32 = 10;
    x += 1; //incrementing x by 1.
    println!("Incrementing x by 1: {}", x);
    x -= 1; //decrementing x by 1. 
    println!("Decrementing x by 1: {}", x);
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Order of Operations");
    println!("-  --  --  --");

    let x : i32 = 10;
    let y : i32 = 5;
    let z : i32 = 2;
    let result : i32 = x + y * z; //multiplication is done first before addition.
    println!("Result of {} + {} * {} is {}", x, y, z, result);
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Using Parentheses");
    println!("-  --  --  --");

    let x : i32 = 10;
    let y : i32 = 5;
    let z : i32 = 2;
    let result : i32 = (x + y) * z; //addition is done first before multiplication.
    println!("Result of ({} + {}) * {} is {}", x, y, z, result);
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Using Variables");
    println!("-  --  --  --");

    let x : i32 = 10;
    let y : i32 = 5;
    let z : i32 = 2;
    let result : i32 = x + y * z; //multiplication is done first before addition.
    let final_result : i32 = result + x; //Using the result of the previous operation in another operation. 
    println!("Result of {} + {} * {} + {} is {}", x, y, z, x, final_result);
    println!("----------------------------------");

}
