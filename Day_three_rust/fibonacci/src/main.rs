fn main() {
    // Using this style will only print the result of the fibonacci number.
    let result = fibonacci(19);
    println!("The result is {}", result);

    //To iterate through the fibonacci sequence from 0 to 10 we do this
    let number = 20;
    for i in 0..number {
        // Using this style will print the fibonacci number only.
        println!("The fibonacci number is: {}", fibonacci(i));
    }
    println!("-------------------");

    for i in 0..number {
        // Using this style will print the fibonacci number and the index of the fibonacci number.
        println!("Fibonacci({}) = {}", i, fibonacci(i));
    }
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

// fn fibonacci(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     }
//     if n == 1 {
//         return 1;
//     }
//     fibonacci(n - 1) + fibonacci(n - 2)
// }

// fabonacci using iteration
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0; // F(0) = 0
    }
    if n == 1 {
        return 1; // F(1) = 1
    }

    let mut a = 0; // F(0)
    let mut b = 1; // F(1)
    let mut c = 0;

    for _ in 2..=n { // Start from 2 up to n
        c = a + b; // Calculate the next Fibonacci number
        a = b;     // Update a to the last Fibonacci number
        b = c;     // Update b to the current Fibonacci number
    }
    c // Return the nth Fibonacci number
}

// The fibonacci function takes an unsigned 32-bit integer n as a parameter and returns an unsigned 32-bit integer.
