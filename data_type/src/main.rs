fn main() {
    //There are 2 categories of data in rust the primitive and coumpound data types 
    //Primitive data types are the most basic data types and they are immutable by default 
    //Coumpound data types are made up of multiple values and they can be mutable or immutable 
    //Primitive data types are: integers, floats, booleans, characters 
    //Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, isize, usize 
    //Floats: f32, f64 
    // In rust integers are definded into 2 categories signed and unsigned integers 
    // signed integers are i8, i16, i32, i64, i128, isize 
    // unsigned integers are u8, u16, u32, u64, u128, usize 
    // isize and usize are dependent on the architecture of the computer and they are either 32 or 64 bits in size 
    // isize is the same size as the memory address of the computer and it can be positive or negative 
    // usize is the same size as the memory address of the computer and it is always positive
    // Floats are f32 and f64 and f64 is the default data type for floats in rust 
    // Boolean is bool and it is either true or false and in some cases 1 or 0 respectively 
    // Character is char and it is a single unicode character and it is 4 bytes in size

    // always add the data type so as to make sure it does not display errors when computation is done on the variable.
    let x:u32 = 5;
    println!("The value of x is: {x}"); // this will not print the value of x because it is not in debug mode and it has to be in scope of the variable to be able to work.
    println!("The value of x is: {:?}", x);  // this will print the value of x in debug mode {:?}
    println!("The value of x is: {}", x); // this will print the value of x in normal mode {} 

    // FLOATS
    let y:f32 = 5.0; 
    let z:f64 = 5.0; // f64 is the default data type for floats
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);


    // Boolean
    let is_true:bool = true; // boolean can be true or false and in some cases 1 or 0 respectively
}
