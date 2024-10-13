// Vectors are similar to arrays but they can grow or shrink in size. 
// Vectors are represented using Vec<T> where T is the type of the elements.
// The Vec<T> type is a generic type that is used to represent a vector of elements of type T.
// Their are two ways to create a vector in Rust:
// 1. Using the vec! macro
// 2. Using the Vec::new() method
// The vec! macro is used to create a vector with elements of the same type.
// The Vec::new() method is used to create an empty vector.
// The push method is used to add an element to the end of a vector.
// The pop method is used to remove the last element from a vector.
// The len method is used to get the length of a vector.
// The is_empty method is used to check if a vector is empty.
// The get method is used to get an element at a specific index.

//Their size is not fixed and can grow or shrink in size as needed.

fn main() {
    let mut vec : Vec<i32> = Vec::new(); //Creating an empty vector of type i32
    let vect = vec![1,2,3,4,5]; //Creating a vector with elements of type i32
    vec.push(6); //Adding an element to the end of the vector
    let my_vec: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10]; //Creating a vector with elements of type i32
    let fisrt = my_vec[0]; //Accessing the first element of the vector
    println!("First element: {}", fisrt);

    let last = my_vec[my_vec.len() - 1]; //Accessing the last element of the vector
    println!("Last element: {}", last);

    if let Some(firs) = vect.get(5) { //Accessing the element at index 5
        println!("element: {}", firs);
    } else {
        println!("No element at index 5");
    }

    for i in &my_vec { //Iterating over the elements of the vector
        println!("{}", i);
    }
}
