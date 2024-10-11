
//talking about custom types in rust
//1. struct 
//2. enum
//3. union
//4. trait
//5. type alias


//1. struct
// struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group of data items into a single unit called a struct type or structure type in Rust.
// struct is a way to create a custom data type in Rust. A struct is a way to group related data together into a single unit called a struct type or structure type in Rust. 

struct Person {
    name: String,
    age: u32,
    is_adult: bool,
}

fn main() {

    //creating an instance of the Person struct
    let person = Person {
        name: String::from("John"),
        age: 30,
        is_adult: true,
    };

    //Incase 2 students have the same name, we can use the clone method to create a new instance of the Person struct with the same name.
    let student = Person {
        name: person.name.clone(),
        age: 30,
        is_adult: true,
    };

    //If the stdent have same data but different name, we can use the reference operator to create a new instance of the Person struct with the same data but different name.
    let student1 = Person {
        name: String::from("Jane"),
        ..person
    };


    //accessing the fields of the Person struct
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Is Adult: {}", person.is_adult);

    //accessing the fields of the Person struct
    println!("Name: {}", student.name);
    println!("Age: {}", student.age);
    println!("Is Adult: {}", student.is_adult);

    //creating an instance of the sudent1 struct
    println!("Name: {}", student1.name);
    println!("Age: {}", student1.age);
    println!("Is Adult: {}", student1.is_adult);

}

