// Primitive data types
// int float, bool, char

// Integer
// Rust has signed (+ and -) and unsigned integer (only+) types of different sizes.
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16, u32, u64, u128: Unsigned integers.
fn main() {
    let x: i32 = -42; // 32-bit signed integer
    let y: u64 = 100; // 64-bit unsigned integer
    println!("x: {}, y: {}", x, y);

    // diff bet i32 (32 bits) and i64(54 bits)
    // range :
    // i32 - 2,147,483,647
    // i64 - 9,223,372,036,854,775,807
    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;
    println!("e: {}, i: {}", e, i);

    // Floats [Floating Point Types]
    // f32, f64
    let pi: f32 = 3.14;
    println!("pi: {}", pi);

    // Boolean Values: true, false
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);

    // Character Type - char
    let letter: char = 'a';
    println!("letter: {}", letter);
}