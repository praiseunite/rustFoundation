fn main() {
    println!("Hello, world!");
    first_function();
    second_function("John");
}

// Functions
print!("-------------------");
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
