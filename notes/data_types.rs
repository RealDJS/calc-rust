// Import necessary modules and crates
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

// Entry point of the Rust program
fn main() {
    // ----------------------------
    // Data Types, Variables, and Syntax
    // ----------------------------

    // Immutable variable (default)
    let x = 5;
    // Mutable variable
    let mut _y = 10;

    // Basic data types
    let _integer: i32 = -42;       // 32-bit signed integer
    let _float: f64 = 3.14;        // 64-bit floating-point number
    let _boolean: bool = true;     // Boolean value
    let _character: char = 'R';    // Character type
    let _string: &str = "Hello, Rust!"; // String slice

    // Arrays and tuples
    let _array: [i32; 3] = [1, 2, 3];        // Fixed-size array
    let tuple: (i32, f64, bool) = (500, 6.4, true); // Tuple with different types

    // Destructuring a tuple
    let (_a, _b, _c) = tuple;

    // Printing to console
    println!("The value of x is: {}", x);

    // ----------------------------
    // Functions and Closures
    // ----------------------------

    // Calling a function with parameters and return value
    let sum = add(5, 7);
    println!("Sum: {}", sum);

    // Calling a function that doesn't return a value
    print_message("Hello from Rust!");

    // Using a closure (anonymous function)
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    let product = multiply(4, 6);
    println!("Product: {}", product);

    // ----------------------------
    // Structs and Enums
    // ----------------------------

    // Creating an instance of a struct
    let point = Point::new(3.0, 4.0);
    println!("Distance from origin: {}", point.distance_from_origin());

    // Using an enum
    let heading = Direction::North;

    // ----------------------------
    // Pattern Matching
    // ----------------------------

    // Matching on an enum
    match heading {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Heading South"),
        Direction::East => println!("Heading East"),
        Direction::West => println!("Heading West"),
    }

    // Matching on Option type
    let maybe_number: Option<i32> = Some(7);
    match maybe_number {
        Some(n) if n > 5 => println!("Got a big number: {}", n),
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }

    // Using if let for concise matching
    if let Some(n) = maybe_number {
        println!("Number is: {}", n);
    }

    // ----------------------------
    // Error Handling
    // ----------------------------

    // Handling errors with match
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading file: {}", e),
    }

    // ----------------------------
    // Modules and Crates
    // ----------------------------

    // Using a function from a module
    let result = math_utils::add(2, 3);
    println!("Result from math_utils::add: {}", result);

    // ----------------------------
    // Expressions, Conditionals, and Loops
    // ----------------------------

    // Using an expression to initialize a variable
    let x = {
        let y = 10;
        y + 5 // Expression evaluates to 15
    };
    println!("Value of x from expression: {}", x);

    // If-else conditionals
    if x > 10 {
        println!("x is greater than 10");
    } else {
        println!("x is 10 or less");
    }

    // For loop over a range
    for i in 0..5 {
        println!("i: {}", i);
    }

    // While loop
    let mut count = 0;
    while count < 5 {
        println!("count: {}", count);
        count += 1;
    }

    // Infinite loop with break and continue
    let mut n = 0;
    loop {
        n += 1;
        if n == 3 {
            continue; // Skip the rest and start next iteration
        }
        if n > 5 {
            break; // Exit the loop
        }
        println!("n: {}", n);
    }

    // ----------------------------
    // Lists and Dictionaries
    // ----------------------------

    // Vectors (growable arrays)
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Iterating over a vector
    for num in &numbers {
        println!("Number in vector: {}", num);
    }

    // HashMap (dictionary)
    let mut scores = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 75);

    // Accessing values in a HashMap
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    // Updating values in a HashMap
    scores.entry("Charlie").or_insert(85);

    // Iterating over a HashMap
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // ----------------------------
    // Input/Output
    // ----------------------------

    // Reading input from the console
    println!("Enter your name:");
    let mut name = String::new(); // Mutable string to store input
    io::stdin()
        .read_line(&mut name) // Read input and store in 'name'
        .expect("Failed to read line");
    println!("Hello, {}!", name.trim()); // trim() removes whitespace

    // Writing to a file
    if let Err(e) = write_to_file() {
        println!("Error writing to file: {}", e);
    }
}

// ----------------------------
// Functions and Closures
// ----------------------------

// Function with parameters and return type
fn add(a: i32, b: i32) -> i32 {
    a + b // Return value without semicolon
}

// Function that doesn't return a value (unit type)
fn print_message(message: &str) {
    println!("{}", message);
}

// ----------------------------
// Structs and Enums
// ----------------------------

// Defining a struct
struct Point {
    x: f64,
    y: f64,
}

// Implementing methods for a struct
impl Point {
    // Associated function (constructor)
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Defining an enum
enum Direction {
    North,
    South,
    East,
    West,
}

// ----------------------------
// Error Handling
// ----------------------------

// Function that reads username from a file
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?; // '?' propagates the error if any
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Function to write to a file
fn write_to_file() -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(b"Hello, file!")?;
    Ok(())
}

// ----------------------------
// Modules and Crates
// ----------------------------

// Declaring a module
mod math_utils {
    // Function to add two numbers
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// Note: To use external crates, add them to Cargo.toml under [dependencies]
// For example:
// [dependencies]
// rand = "0.8.4"
