// Data Types

// Scaler data types
    // represent single value

fn main() {
    // integers
    // let x: i32 = 98_222; // Decimal
    // let x: i32 = 0xff; // Hex
    // let x: i32 = 0o77; // Octal
    // let x: i32 = 0b1111_0000; // Binary
    // let c: u8 = b'A'; // Byte

    // // println!("{}", c);
    // // // floating-point numbers
    // let f: f64 = 2.0;
    // let g: f32 = 3.0;
    // let sum: f64 = 11.2 + 54.2;
    // let sub: f64 = 54.00 - 10.00;
    // let product: f64 = 2.10 * 1.1;
    // let quotient: f64 = 2.10 / 1.1;
    // let remainder: f64 = 43 % 5;
    // println!("{}", product);

    // // booleans
    // let sahi: bool = true;
    // let galat: bool = false;

    // // character
    // let char = "Z";




    // Compound data types

    // Tuples
    let tup = ("harshiiet", 25);
    let (name, age) = tup;

    println!("Th guy's name is {} and age is {}", name, age);

    // accessing value from tuple with dot notation
    let age = tup.1;
    println!("{}", age);

    // Arrays
    let error_codes = [200, 404, 502];

    // access values with their index value
    let not_found = error_codes[1];
    println!("Error {}", not_found);


    let arr = [0; 8]; // another way of making an array (this says create 8 items with value 0) 
    println!("{}", arr[2]);
}
