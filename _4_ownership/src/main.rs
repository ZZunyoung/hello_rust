// Ownership, Borrowing and References

// Ownership
// -----------
//C, C++ -> Memory Management control Issue
// Garbage Collector solved this issue, but created a new issue
// [stopping/resuming the program]
// OWNERSHIP introduced by Rust to solve memory safety issues and high performance at the same time.
// What is Ownership?
// Every value has a single owner [every variable has one value, and it is its sole owner]

// Ownership Rules
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Example: Rule_1
// fn main() {
//     let s1 = String::from("RUST");
//     // let len = calculate_length(&s1);
//     // println!("The length of '{}' is {}.", s1, len);
// }

// Rule_2
// fn main() {
//     let s2 = s1; // s1 is moved to s2, s1
//     // println!("{}", s1); // error: value borrowed here after move
//     println!("{}", s2); // works fine
// }

// Rule_3
// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }
// // s1 goes out of scope here, and the memory allocated for it is freed

// fn print_lost(s: &String) {
//     println!("The value of s is: {}", &s1);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }