// Compund Data Types
// arrays, tuples, slices, and strings (slice string)

// Arrays
fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers: {:?}", numbers);
    // let mix = [1, 2, 'a', true];
    // println!("Mix: {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);
    println!("Fruits 1st element: {}", fruits[0]);
    println!("Fruits 2nd element: {}", fruits[1]);
    println!("Fruits 3rd element: {}", fruits[2]);

    // Tuples
    let person: (String, i32, bool) = (("Alice").to_string(), 30, true);
    println!("Person: {:?}", person);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is student: {}", person.2);

    let mix_tuple = ("Alice", 30, true, [1, 2, 3]);
    println!("Mix tuple: {:?}", mix_tuple);

    // Slices: [1, 2, 3, 4, 5]
    let numbers_slice:&[i32] = &[1, 2, 3, 4, 5];
    println!("Numbers slice: {:?}", numbers_slice);

    let animal_slice: &[&str] = &["cat", "dog", "rabbit"];
    println!("Animal slice: {:?}", animal_slice);

    let book_slice: &[&String] = &[&"IT".to_string(), &"The Shining".to_string(), &"Misery".to_string()];
    println!("Book slice: {:?}", book_slice);

    // Strings Vs String Slices (&str)
    // Strings [growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (string slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice: {}", slice);
}