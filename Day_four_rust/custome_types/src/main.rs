
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

impl Person {  //impl is a keyword that is used to implement methods on a struct type.
    fn introduce(&self) { //The &self parameter is a reference to the instance of the struct on which the method is called. e.g self.name is the name of the instance of the struct.
        println!("Hi, my name is {} and I am {} years old.", self.name, self.age);
    }
}

//tuples are a way to group together multiple values of different types into a single unit called a tuple type in Rust.
// tuples struct is a way to create a custom data type in Rust. A tuple struct is a way to group related data together into a single unit called a tuple struct type in Rust.

#[derive(Debug)] //The derive(Debug) annotation is used to automatically implement the Debug trait for the Color tuple struct. The Debug trait is used to print the value of the Color tuple struct using the {:?} format specifier.
struct Color(i32, i32, i32);

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


    //calling the introduce method on the person instance
    person.introduce();
    student.introduce();
    student1.introduce();

    //creating an instance of the Color tuple struct
    let color = Color(255, 0, 0); //red color
    println!("Red: {}, Green: {}, Blue: {}", color.0, color.1, color.2);
    println!("the value {:?}", color);



}

