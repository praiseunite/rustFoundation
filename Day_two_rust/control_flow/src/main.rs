fn main() {
    // < >  <=  >=  ==  != &&  ||  ! if  else  else if match loop  while  for break  continue return.
    // Logical operators are used to compare two values and return a boolean value (true or false) based on the comparison result of the two values being compared.
    // The logical operators in Rust programming language are as follows:
    // < (less than) - returns true if the value on the left is less than the value on the right.
    // > (greater than) - returns true if the value on the left is greater than the value on the right.
    // <= (less than or equal to) - returns true if the value on the left is less than or equal to the value on the right.
    // >= (greater than or equal to) - returns true if the value on the left is greater than or equal to the value on the right.
    // == (equal to) - returns true if the value on the left is equal to the value on the right.
    // != (not equal to) - returns true if the value on the left is not equal to the value on the right.
    // && (logical AND) - returns true if both values on the left and right are true.
    // || (logical OR) - returns true if either value on the left or right is true.
    // ! (logical NOT) - returns true if the value is false and false if the value is true.
    // The logical operators are used to compare two values and return a boolean value (true or false) based on the comparison result of the two values being compared.

    // if else statement is used to execute a block of code if a condition is true and another block of code if the condition is false.
    // The if else statement in Rust programming language is as follows:
    // if condition { 
    //     // block of code to be executed if the condition is true
    // } else {
    //     // block of code to be executed if the condition is false
    // }
println!("----------------------------------");
println!("Greater than with if else statement");
println!("-  --  --  --");

    let condition = 2 > 3; 
    println!("The condition is {}", condition); //prints false because 2 is not greater than 3.

    println!("-  --  --  --");

    if condition {
        println!("2 is greater than 3"); //this block of code will not be executed because the condition is false.
    } else {
        println!("2 is not greater than 3"); 
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Less than with if else statement");
    println!("-  --  --  --");

    let condition = 2 < 3;
    println!("The condition is {}", condition); //prints true because 2 is less than 3.

    println!("-  --  --  --");

    if condition {
        println!("2 is less than 3"); 
    } else {
        println!("2 is not less than 3"); //this block of code will not be executed because the condition is true.
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Greater than or equal to with if else statement");
    println!("-  --  --  --");

    let condition = 2 >= 3;
    println!("The condition is {}", condition); //prints false because 2 is not greater than or equal to 3.

    println!("-  --  --  --");

    if condition {
        println!("2 is greater than or equal to 3"); //this block of code will not be executed because the condition is false.
    } else {
        println!("2 is not greater than or equal to 3"); 
    }
    println!("----------------------------------");

    println!("----------------------------------");
    println!("Less than or equal to with if else statement");
    println!("-  --  --  --");

    let condition = 2 <= 3;
    println!("The condition is {}", condition); //prints true because 2 is less than or equal to 3.

    println!("-  --  --  --");

    if condition {
        println!("2 is less than or equal to 3"); 
    } else {
        println!("2 is not less than or equal to 3"); //this block of code will not be executed because the condition is true.
    }
    println!("----------------------------------");

    println!("----------------------------------");
    println!("Equal to with if else statement");
    println!("-  --  --  --");

    let condition = 2 == 3;
    println!("The condition is {}", condition); //prints false because 2 is not equal to 3.

    println!("-  --  --  --");

    if condition {
        println!("2 is equal to 3"); //this block of code will not be executed because the condition is false.
    } else {
        println!("2 is not equal to 3"); 
    }
    println!("----------------------------------");

    println!("----------------------------------");
    println!("Not equal to with if else statement");
    println!("-  --  --  --");

    let condition = 2 != 3;
    println!("The condition is {}", condition); //prints true because 2 is not equal to 3.

    println!("-  --  --  --");

    if condition {
        println!("2 is not equal to 3"); 
    } else {
        println!("2 is equal to 3"); //this block of code will not be executed because the condition is true.
    }
    println!("----------------------------------");

    println!("----------------------------------");
    println!("Logical AND with if else statement");
    println!("-  --  --  --");

    let condition = 2 < 3 && 2 > 1;
    println!("The condition is {}", condition); //prints true because both conditions are true.

    println!("-  --  --  --");

    if condition {
        println!("2 is less than 3 and 2 is greater than 1"); 
    } else {
        println!("Either 2 is not less than 3 or 2 is not greater than 1"); //this block of code will not be executed because the condition is true.
    }
    println!("----------------------------------");

    println!("----------------------------------");
    println!("Logical OR with if else statement");
    println!("-  --  --  --");
    //In Rust, logical && (AND) has higher precedence than logical || (OR), so the && operation is evaluated first.

    let condition = 2 < 3 || 2 > 1;
    let situation = 2 < 3 || 2 < 1 && 2 > 1; //this is an example of logical OR and logical AND operators used together.
    println!("The situation is {}", situation); //prints true because at least one condition is true.
    println!("The condition is {}", condition); //prints true because at least one condition is true.

    println!("-  --  --  --");

    if condition {
        println!("Either 2 is less than 3 or 2 is greater than 1"); 
    } else {
        println!("2 is not less than 3 and 2 is not greater than 1"); //this block of code will not be executed because the condition is true.
    }
    println!("----------------------------------");

    println!("----------------------------------");
    println!("Logical NOT with if else statement");

    let condition = !(2 < 3);
    println!("The condition is {}", condition); //prints false because the condition is true.

    println!("-  --  --  --");

    if condition {
        println!("2 is not less than 3"); //this block of code will not be executed because the condition is false.
    } else {
        println!("2 is less than 3"); 
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("If else if statement");
    println!("-  --  --  --");

    let x = 10;
    let y = 10;

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x is equal to y"); //this block of code will be executed because x is equal to y.
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Match statement");
    println!("-  --  --  --");

    let x = 10;

    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3 => println!("x is 3"),
        4 => println!("x is 4"),
        5 => println!("x is 5"),
        _ => println!("x is not 1, 2, 3, 4, or 5"), //this block of code will be executed because x is not 1, 2, 3, 4, or 5.
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Loop statement");
    println!("-  --  --  --");

    let mut x = 0;

    loop {
        println!("x is {}", x);
        x += 1;

        if x == 5 {
            break; //this statement will break the loop when x is equal to 5.
        }
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("While statement");
    println!("-  --  --  --");

    let mut x = 0;

    while x < 5 {
        println!("x is {}", x);
        x += 1;
    }

    let mut y = 10;

    while y != 0{
        println!("y is {}", y);
        y -= 1;
    }
    println!("y is now 0");
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("For statement");
    println!("-  --  --  --");

    for x in 0..5 {
        println!("x is {}", x);
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Break statement");
    println!("-  --  --  --");

    for x in 0..5 {
        if x == 3 {
            break; //this statement will break the loop when x is equal to 3.
        }
        println!("x is {}", x);
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Continue statement");
    println!("-  --  --  --");

    for x in 0..5 {
        if x == 3 {
            continue; //this statement will skip the iteration when x is equal to 3.
        }
        println!("x is {}", x);
    }
    println!("----------------------------------");
    println!("  ");

    println!("----------------------------------");
    println!("Loop");

    let mut counter = 0;
    let result = loop { //loop is an expression that returns a value.
        println!("Counter is {}", counter);
        counter += 1;
        if counter == 5 {
            // break; //this statement will break the loop when counter is equal to 5.
            break counter * 3; //this statement will break the loop when counter is equal to 5 and return the value of counter multiplied by 3. without value, it will throw an error.
        }
        println!("Printing at second line so counter is {}", counter);
    };
    println!("result is {result}"); //prints () because loop is an expression that returns a value.
    println!("----------------------------------");
}
