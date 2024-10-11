//importing the some_module module
mod lib;
// Title: Modules in Rust Programming Language
// Introduction: This is a simple program that demonstrates how to use modules in Rust programming language.
mod members{
    //defining the functions in the members module

    // If you do not add the pub keyword before the function, the function will not be accessible outside the module.
    pub fn adding_numbers(){
        println!("Adding members");
    }
    pub fn remove_numbers(){
        println!("Adding members");
    }
}

fn main() {
    //calling the functions from the members module.
    members::adding_numbers();
    members::remove_numbers();
    //calling the functions from the some module.
    lib::some::adding_numbers();
    lib::some::remove_numbers();
    lib::some::special::numbers();

    //name of file, name of module, name of function
    //The path to the function is composed of the name of the file, the name of the module, and the name of the function.

}

//crates and modules
//Crates are a collection of Rust source code files. A crate can contain one or more modules. 
//Modules are a way to organize code in Rust. Modules allow you to group related code together. 