#![allow(warnings)]
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);

    let mut x = 1;
    x += 2; 
    
    assert_eq!(x, 3);

    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    let y = 7;
    println!("Outer scope value of x is {} and value of y is {}", x, y); 

    define_x();
    
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
    
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    let _x = _x;


    let _y: i32 = 4;
    // Shadowing
    let _y = "I can also be bound to text!"; 

    let (mut x, mut y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    let (x, y);
    (x,_) = (3, 4);
    [_, y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

}

fn define_x() {
    let x = "hello";

    println!("{}, world", x)
}