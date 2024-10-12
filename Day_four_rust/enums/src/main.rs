#[derive(Debug)] //this is to help us print out our enum values.
// Always add the # to the #[derive(Debug)]
//enum are used to create custom data types 
//enums are used to create a type that can be one of several variants 
enum Message{
    Quit,
    Move{x: i32, y:i32},
    Write(String),
    ChangeColour(i32, i32, i32),
}

//pattern matching
//pattern matching is a way to compare a value against a series of patterns and then execute code based on which pattern matches.
//pattern matching is a powerful feature in Rust that allows you to match against different patterns and execute code based on which pattern matches.
//pattern matching is a way to destructure a value and extract the data from it.
//It is similar to the switch statement in other programming languages.

fn match_message(msg: Message){
    match msg{
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move{x, y} => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColour(r, g, b) => {
            println!("Change the colour to red: {}, green: {}, blue: {}", r, g, b);
        }
    }
}

//option is a type that represents an optional value.
//option is a type that represents a value that may or may not be present.

//The Option enum has two variants: Some and None. 
//The Some variant holds a value, while the None variant does not hold a value and represents the absence of a value.
//The Option enum is used to handle the case where a value may or may not be present.

fn get_value(x: i32) -> Option<i32>{
    if x % 2 == 0{
        Some(x)
    }else{
        None
    }
}

fn safe_divide(x: i32, y: i32) -> Option<f32>{
    if y == 0{
        None
    }else{
        Some(x as f32 /y as f32)
    }
}

fn main() {

    //creating an instance of the Message enum
    let msg = Message::Write(String::from("Hello, world!"));
    let msg1 = Message::Quit;
    // let msg2 = Message::Move{10, 20}; // use curly braces to specify the field values.  
    let msg3 = Message::Move{x: 7, y: 40}; // use curly braces to specify the field names
    let msg4 = Message::ChangeColour(255, 0, 0);

    println!("{:?}", msg);
    println!("{:?}", msg1);
    // println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);

    //calling the match_message function
    println!("Calling the match_message function");
    match_message(msg);
    match_message(msg1);
    match_message(msg3);
    match_message(msg4);

    //calling the get_value function
    println!("Calling the get_value function");
    let x = 10;
    match get_value(x){
        Some(value) => println!("The value is {}", value),
        None => println!("The value is None"),
    }

    let y = 15; //odd number
    match get_value(y){
        Some(value) => println!("The value is {}", value),
        None => println!("The value is None"),
    }

    //calling the safe_divide function
    println!("Calling the safe_divide function");
    let x = 10;
    let y = 2;
    match safe_divide(x, y){
        Some(value) => println!("The result is {}", value),
        None => println!("Cannot divide by zero"),
    }

    // This is used when you want to use the switch/pattern statement and it gives you a meassage. You can as well, declear the pattern in the main or outside as a method.
    let x = 10;
    let y = 0;
    match safe_divide(x, y){
        Some(value) => println!("The result is {}", value),
        None => println!("Cannot divide by zero"), //Cannot divide by zero is printed out because we cannot divide by zero.
    }

    let result = safe_divide(10, 2);
    println!("{:?}", result); //Some(5.0) is printed out because 10 divided by 2 is 5. 

    let result = safe_divide(10, 0); 
    println!("{:?}", result); //None is printed out because we cannot divide by zero.
}
