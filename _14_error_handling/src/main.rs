// Error Handling techniques [2 approaches]
fn main() {
    // Approach 1
    // enum Option<T>{ // Define the generic Option type
    //     Some(T), // Represents a value
    //     None, // Represents no value
    // }

    fn divide_option(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        }
        else {
            Some(numerator / denominator)
        }
    }

    match divide_option(10.0, 0.0) {
        Some(x) => println!("result: {}", x),
        None => println!("Canot divide by Zero!"),
    }

    // // Approach 2
    // enum Result<T, E> { // Define the generic Result type
    //     Ok(T), // Represents a value
    //     Err(E), // Represents an error
    // }

    fn divide_result(numerator: f64, denomivator: f64) -> Result<f64, String> {
        if denomivator == 0.0 {
            Err("Cannot divide by 0".to_string())
        }
        else {
            Ok(numerator / denomivator)
        }
    }
    match divide_result(100.23, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
