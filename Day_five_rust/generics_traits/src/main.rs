// Generics are a way to make code more flexible and reusable. They are a way to write code that works with multiple types without duplicating code.
// std::fmt::Display is a trait that is used to format a value using the {} format specifier. The Display trait is used to print a value using the {} format specifier.
fn print_item<T: std::fmt::Display>(items: &[T]) {
   for item in items {
        println!("{}", item);
   }
}

// Traits are a way to define shared behavior between different types. They are a way to define a set of methods that a type must implement in order to use the trait.
trait Speak {
    fn speak(&self) -> String; //A trait method is a method that is defined in a trait. It is a way to define the behavior of a type using the trait.
    // &self is a reference to the instance of the struct on which the method is called. e.g self.name is the name of the instance of the struct.
}

// Implementing a trait for a type is a way to define the behavior of the type using the trait. It is a way to define the methods that the type must implement in order to use the trait.
struct Human;
impl Speak for Human {
    fn speak(&self) -> String {
        String::from("Hello, I am a human."); //The String::from method is used to create a new String type from a string slice.
        "Hello, I am a human.".to_string() //The to_string method is used to convert a string slice to a String type.
    }
}

struct Dog;
impl Speak for Dog {
    fn speak(&self) -> String {
        String::from("Woof, I am a dog.");
        "Woof, I am a dog.".to_string()
    }
}


fn main() {
    let numbers = [1,3,4,2,5,6,3];
    let names = ["John", "Jane", "Doe", "Smith", "Alice", "Bob"];

    print_item(&numbers);
    print_item(&names);

    let human = Human; //Creating an instance of the Human struct
    let dog = Dog; //Creating an instance of the Dog struct 

    println!("{}", human.speak()); //Calling the speak method on the human instance
    println!("{}", dog.speak()); //Calling the speak method on the dog instance
}
