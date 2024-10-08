fn main() {
    let x = 5; // while using let, they are immutable by default you can't change the value of x
    // x = 7; will not work.  to change the value of x you have to use mut keyword
    let mut y = 5;
    y = 7; // this will work.
    // println!("The value of x is: {}", x);
    println!("The value of x is: {x}");
    println!("Hello, world!");

    //Showdowing allows you to reuse the same name for a variable and then
    //change it value by shadowing it. and when called, the last value will be printed.
    let a = 5;
    let a = 6;
    println!("The value of a is: {}", a);

    //constants are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    const MINUTES_IN_HOUR: u32 = 60;
    const HOURS_IN_DAY: u32 = 24;
    const DAYS_IN_YEAR: u32 = 365;
    const MINUTES_IN_YEAR: u32 = MINUTES_IN_HOUR * HOURS_IN_DAY * DAYS_IN_YEAR;
    println!("The value of MINUTES_IN_YEAR is: {}", MINUTES_IN_YEAR);

}
