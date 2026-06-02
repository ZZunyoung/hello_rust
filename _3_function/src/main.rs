// Functions
// Entry point
// an function / variables shoud be written in snake case
// snake case: hello_world
// kebab case: hello-world
fn main() {
    hello();
    tell_height(170);
    human_id("Alice", 30, 165.5);
    let _X = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Total price: {}", _X);
    let y = add(3, 4);
    println!("Sum: {}", y);
    println!("Sum: {}", add(10, 20));

    let weight = 70.0; // in kg
    let height = 1.75; // in meters
    let bmi = calculate_bmi(weight, height);
    println!("BMI: {:.2}", bmi);
}

// Hoisting - can call function anywhere in your code
fn hello() {
    println!("Hello, Rust!");
}

// you can insert input values
fn tell_height(height: u32) {
    println!("Height: {} cm", height);
}

// you can insert more than on value
fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}, Age: {}, Height: {} cm", name, age, height);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value
// Statement: Anything that does not return a value
// Almost all statements end with a semicolon (;)

//Expression
// ------------------------
// 5
// true & false
// add(3, 4)
// if condition { value1 } else { value2 }
// ({code})

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}