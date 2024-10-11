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

}
