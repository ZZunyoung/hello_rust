use std::collections::HashMap;

// Collection Types:
// Vectors - UTF8 - Hashmaps
fn main() {
    // Vectors

    // let _v: Vec<i32> = Vec::new();

    // // Macro to create a vector of numbers
    // let mut _v: Vec<i32> = Vec::new();

    // let mut _v: Vec<i32> = vec![1,2,3];

    // _v.push(5);
    // _v.push(6);
    // _v.push(7);
    // _v.push(8);

    // println!("{:?}", _v)

    let _v = vec![1,2,3,4,5];

    // let third = &_v[2]; // Direct indexing

    // println!("The third element is {third}");

    let third = _v.get(9);
    match third{
        Some(third) => println!("The third element for a GET method is {third}"),
        None => println!("There is no third element.")
    }

    // UTF-8
    // 1
    let _s = "whatever".to_string();
    // 2
    let _s = String::from("whatever");
    // Mutate the variable [push to it]
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    println!("the value of s = {}", s);

    let salam = String::from("안녕");
    let salut = String::from("경례");

    // If you want to combine strings, use the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("The value of s3 = {s3}");

    // Formatting Strings
    let full_message = format!("{salam} {salut}");
    println!("{full_message}");

    // Hashmpas
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}")
    }
    println!("score: {score}");
}
