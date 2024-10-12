//Error handling in Rust is done using the Result enum. The Result enum has two variants: Ok and Err.

fn get_value(x: i32) -> Result<i32, String>{
    if x % 2 == 0{
        Ok(x)
    }else{
        Err(String::from("Value is not even"))
    }
}

fn check_password(password: &str) -> Result<(), String>{
    if password.len() >= 8{
        Ok(()) 
    }else{
        Err(String::from("Password is too short".to_string()))  //The Err variant holds an error message of type String.
    }
}
// enum Result<T, E>{
//     Ok(T), //The Ok variant holds a value of type T. The Ok variant is used to represent the case where the operation was successful and a value of type T is returned.
//     Err(E), //The Err variant holds a value of type E. The Err variant is used to represent the case where the operation failed and an error of type E is returned.
// }

fn main() {
    //calling the check_password function
    let password = "passw";
    match check_password(password){
        Ok(_) => println!("Password is valid"),
        // Ok(()) => println!("Password is valid"), // The underscore (_) is used to ignore the value of the Ok variant.
        Err(msg) => println!("Error: {}", msg),
    }
}
