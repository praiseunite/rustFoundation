// Generics are a way to make code more flexible and reusable. They are a way to write code that works with multiple types without duplicating code.

fn print_item<T: std::fmt::Display>(items: &[T]) {
   for item in items {
        println!("{}", item);
   }
}

fn main() {
    let numbers = [1,3,4,2,5,6,3];
    let names = ["John", "Jane", "Doe", "Smith", "Alice", "Bob"];

    print_item(&numbers);
    print_item(&names);

}
