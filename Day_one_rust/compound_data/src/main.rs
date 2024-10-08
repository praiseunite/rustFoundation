fn main() {
    //Compound data types are made up of multiple values and they can be mutable or immutable
    //Compound data types are: tuples, arrays, strings 

    // --------------------------------------------------------
    //Tuples are a collection of values of different data types and they are immutable
    let tup:(i32, f64, u8) = (500, 6.4, 1); // the tuples can either be positive or negative and they can be of different data types
    let (x, y, z) = tup; // destructuring the tuple

    //Accessing the values of the tuple
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    //Accessing the values of the tuple
    let x = tup.0; // accessing the value of x in the tuple by destructuring.
    println!("The value of x is: {}", tup.0); // accessing the value of x in the tuple
    println!("The value of y is: {}", tup.1); // accessing the value of y in the tuple
    println!("The value of z is: {}", tup.2); // accessing the value of z in the tuple

    // --------------------------------------------------------
    println!("Hello, world!");
}
