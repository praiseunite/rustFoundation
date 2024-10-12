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

}
