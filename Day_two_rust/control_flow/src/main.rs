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

    let condition = 2 > 3; 
    println!("The condition is {}", condition); //prints false because 2 is not greater than 3.
    if condition {
        println!("2 is greater than 3"); //this block of code will not be executed because the condition is false.
    } else {
        println!("2 is not greater than 3"); 
    }
}
