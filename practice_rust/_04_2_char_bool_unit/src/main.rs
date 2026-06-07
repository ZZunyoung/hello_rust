#![allow(warnings)]
use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    let c1 = '中'; // '' is char, "" is String
    print_char(c1);

    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    // Char: Single charactor of size 4 bytes
    // Bool: Boolean value of size 1 byte
    // Unit: Empty tuple of size 0 bytes, used to return nothing in expression or function
}

fn print_char(c : char) {
    println!("{}", c);
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }