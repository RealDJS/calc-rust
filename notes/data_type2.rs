// Rust Cheat Sheet
// =================

// DATA TYPES, VARIABLES, AND SYNTAX
// ----------------------------------

// Variables in Rust are immutable by default.
// Use `let` to declare a variable.
let x = 5; // Immutable variable of type i32 (default integer type)

// To make a variable mutable, use `mut`.
let mut y = 10; // Mutable variable

// Basic data types

// Integer types: i8, i16, i32, i64, i128 (signed), u8, u16, u32, u64, u128 (unsigned)
let integer: i32 = 42;       // 32-bit signed integer

// Floating point types: f32, f64
let float_num: f64 = 3.14;   // 64-bit floating point number

// Boolean type
let boolean: bool = true;    // Boolean value true or false

// Character type
let character: char = 'A';   // Character type represents a Unicode scalar value

// Strings
let string_literal = "Hello, world!"; // String slice (&str), immutable
let mut string = String::from("Hello"); // Growable, heap-allocated String

// Type inference: Rust can often infer the type based on the context
let z = x + y; // z is inferred to be i32

// LOOPS
// ------

// `loop` is an infinite loop unless `break` is used
let mut count = 0;
loop {
    count += 1;
    if count == 5 {
        break; // Exit the loop when count equals 5
    }
}

// `while` loop executes as long as the condition is true
while count < 10 {
    count += 1;
}

// `for` loop over a range
for i in 0..5 { // Range from 0 inclusive to 5 exclusive
    println!("i = {}", i);
}

// FUNCTIONS AND CLOSURES
// -----------------------

// Function definition with parameters and return type
fn add(a: i32, b: i32) -> i32 {
    a + b // Return value (expression without semicolon)
}

// Calling a function
let sum = add(2, 3); // sum equals 5

// Closures: anonymous functions that can capture variables from the environment
let multiply = |x: i32, y: i32| x * y; // Define a closure
let product = multiply(4, 5); // product equals 20

// Closures can also be stored and passed around
fn apply<F>(f: F) where F: Fn(i32) -> i32 {
    println!("Result: {}", f(3));
}

apply(|x| x + 1); // Pass a closure that increments by 1

// THE VEC (LIST) DATA STRUCTURE
// ------------------------------

// Creating a new vector (dynamic array)
let mut numbers: Vec<i32> = Vec::new(); // Explicit type annotation

// Alternatively, using the vec! macro
let mut numbers = vec![1, 2, 3]; // Creates a vector with initial elements

// Pushing elements into the vector
numbers.push(4); // Adds 4 to the end of the vector

// Accessing elements
let first = numbers[0]; // Access by index (zero-based)

// Iterating over a vector by reference to avoid moving the elements
for number in &numbers {
    println!("Number: {}", number);
}

// Iterating over mutable references to modify elements
for number in &mut numbers {
    *number += 10; // Dereference to modify the value
}

// THE MATCH KEYWORD
// ------------------

// `match` is Rust's powerful pattern matching construct
let number = 3;
match number {
    1 => println!("One"),
    2 | 3 | 5 | 7 => println!("Prime"), // Matches any of the listed values
    13..=19 => println!("Teen"), // Range inclusive (13 to 19)
    _ => println!("Other"), // The underscore `_` is a catch-all pattern
}

// Matching with multiple patterns and binding variables
let pair = (0, -2);
match pair {
    (0, y) => println!("First is zero, y is {}", y),
    (x, 0) => println!("y is zero, x is {}", x),
    _ => println!("Neither is zero"),
}

// STRUCTS AND ENUMS
// ------------------

// Defining a struct (similar to a class without methods)
struct Point {
    x: i32,
    y: i32,
}

// Implementing methods for a struct
impl Point {
    // Associated function (constructor)
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Method that computes the distance from origin
    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

// Creating an instance of a struct
let point = Point::new(10, 20);

// Accessing fields
println!("Point x: {}, y: {}", point.x, point.y);

// Calling a method
println!("Distance from origin: {}", point.distance_from_origin());

// Defining an enum with variants
enum Direction {
    North,
    South,
    East,
    West,
}

// Enums can also hold data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Using an enum with match
let dir = Direction::North;
match dir {
    Direction::North => println!("Going North"),
    Direction::South => println!("Going South"),
    Direction::East => println!("Going East"),
    Direction::West => println!("Going West"),
}

// Matching on enum with data
let msg = Message::Move { x: 10, y: 20 };
match msg {
    Message::Quit => println!("Quit message"),
    Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b),
}

// INPUT/OUTPUT
// -------------

use std::io; // Import the io module for input/output

// Reading input from the user
let mut input = String::new(); // Create a mutable String to hold the input
println!("Enter your name:");
io::stdin() // Get the standard input handle
    .read_line(&mut input) // Read a line into the input String
    .expect("Failed to read line"); // Handle potential errors

// Trim the newline character from the input
let name = input.trim();
println!("Hello, {}!", name);

// File I/O example (writing to a file)
use std::fs::File;
use std::io::Write;

let mut file = File::create("output.txt").expect("Could not create file");
file.write_all(b"Hello, file!") // Write bytes to the file
    .expect("Write failed");

// Reading from a file
use std::fs;

let contents = fs::read_to_string("output.txt")
    .expect("Something went wrong reading the file");

println!("File contents: {}", contents);
