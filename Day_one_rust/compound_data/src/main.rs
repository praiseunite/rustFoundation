fn main() {
    //Compound data types are made up of multiple values and they can be mutable or immutable
    //Compound data types are: tuples, arrays, strings 

    println!(" -------------------------------------------------------- ");
    println!(" Tuples ");
    //Tuples 
    //Tuples are a collection of values of different data types and they are immutable
    let tup:(i32, f64, u8) = (500, 6.4, 1); // the tuples can either be positive or negative and they can be of different data types
    let (x, y, z) = tup; // destructuring the tuple

    // Adding values to a tuples
    let my_tuple: (i32, &str, bool); // Declares a mutable tuple type
    my_tuple = (42, "Rust", true); // Now assigning values
    let (x, y, z) = my_tuple; // Destructuring the tuple
    println!("The value of x is: {:?}", my_tuple); // Accessing the values of the tuple ant this has to be in debug mode {:?}



    //Accessing the values of the tuple
    
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    //Accessing the values of the tuple
    let x = tup.0; // accessing the value of x in the tuple by destructuring.
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", tup.0); // accessing the value of x in the tuple
    println!("The value of y is: {}", tup.1); // accessing the value of y in the tuple
    println!("The value of z is: {}", tup.2); // accessing the value of z in the tuple

    println!(" -------------------------------------------------------- ");

    println!(" -------------------------------------------------------- ");
    println!(" Arrays ");
    //Arrays
    //Arrays are a collection of values of the same data type and they are immutable by default 
    //Arrays are fixed in size and they are stored in the stack memory 
    //Arrays are useful when you want to store a collection of values of the same data type and you know the number of elements in the array 
    let arr = [1, 2, 3, 4, 5]; // the array can either be positive or negative and they are of the same data type
    let arr = [3; 5]; // this will create an array of 5 elements with the value of 3 in each element 
    let first = arr[0]; // accessing the first element in the array
    let second = arr[1]; // accessing the second element in the array
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    // declearing an array of 5 elements with the value of 3 in each element
    let  my_array: [i32; 5]; // Step 1: Declare the array without initializing it
    my_array = [1, 2, 3, 4, 5]; // Step 2: Initialize the array

    println!("The array is: {:?}", my_array); // Print the whole array
    
    // Destructure the array to print individual values (optional)
    let [a, b, c, d, e] = my_array;
    println!("First element: {}", a);

    println!(" -------------------------------------------------------- ");
    
    println!(" -------------------------------------------------------- ");
    println!(" Dynamic size arrays ");
    // Dynamic size arrays 
    // Rust does not have dynamic size arrays but you can use vectors to create dynamic size arrays
    // Vectors are similar to arrays but they can grow or shrink in size 
    // Vectors are stored in the heap memory
    // Vectors are defined in the standard library and you have to import the standard library to use vectors
    let mut my_vec: Vec<i32>; // Step 1: Declare a vector

    my_vec = vec![1, 2, 3, 4, 5]; // Step 2: Initialize with values

    // Print the entire vector
    println!("The vector is: {:?}", my_vec); 

    // Add an element to the vector
    my_vec.push(6);
    println!("Updated vector: {:?}", my_vec);
    println!(" -------------------------------------------------------- ");



    println!("Hello, world!");
}
