// Shadowing
// Shadowing is not the same as marking a variable as mutable.

fn main() {
    let x = 5;
    
    let x = x + 1; // Shadowing: x is shadowed by a new variable with the same name
    {
        let x = x * 2; // Shadowing: x is shadowed again in this inner scope
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x in the outer scope is: {}", x); // 6
}
