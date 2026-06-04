// If Else [ If expression ] [ Else expression ]
#![allow(warnings)] // to ignore warnings for funning warnings
fn main() {
    // let age: u16 = 20;

    // if age >= 18 {
    //     println!("You are an adult.");
    // } else {
    //     println!("You are a minor.");
    // }

    // Multiple conditions with else if:
    // let number = 7;
    
    // if number % 4 == 0 {
    //     println!("{} is divisible by 4.", number);
    // } else if number % 3 == 0 {
    //     println!("{} is divisible by 3.", number);
    // } else if number % 2 == 0 {
    //     println!("{} is divisible by 2.", number);
    // } else {
    //     println!("{} is not divisible by 4, 3, or 2.", number);
    // }

    // Using if in a let statement
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
