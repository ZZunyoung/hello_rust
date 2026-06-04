fn main() {
    // Loop keyword
    // loop {
    //     println!("This will loop forever!");
    //     break; // to exit the loop
    // }

    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 20 {
            break count - 100; // returning a value from the loop
        }
    };
    println!("The result is: {}", result);

    // Loop Lables to Disambiguate Between Multiple Loops
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i * j > 6 {
                break 'outer;
            }
            println!("{} {}", i, j);
        }
    }

    // While Loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // # Looping Through a Collection with for

    let a = [10, 20, 30, 40, 50];
    let b = ["a", "b", "c", "d", "e"];
    for element in a {
        println!("the value is: {}", element);
    }
    for element in b {
        println!("the value is: {}", element);
    }
}
