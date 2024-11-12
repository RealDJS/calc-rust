use std::io;

fn
main(){
    
    let input: String = get_input();

    let val_1: f32 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number.");
            return;
        }
    };

    let sum = add(val_1,20.2);
    println!("{}",sum);

    let sub = subtract(val_1, 4.0);
    println!("{}",sub);

    let product = multiply(val_1,6.0);
    println!("{}", product);

    let div = divide(val_1,7.0);
    println!("{}",div);

    let u_input = "abc123xyz"; 
    let u_list = get_chars(u_input.to_string());

    println!("{:?}",u_list);

} 


// Get expression from the user.
fn get_input() -> String{
    
    println!("Enter an expression: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}


// Parse input to make a list.
fn get_chars(input: String) -> Vec<char> {
    let mut characters = Vec::new();

    for ch in input.chars(){
        characters.push(ch);
    }
    characters
}

// Add two values to create one sum.
fn add(val_1: f32, val_2: f32) -> f32{
    val_1 + val_2
}


// Subtract val_2 from val_1.
fn subtract(val_1: f32, val_2: f32) -> f32{
    val_1 - val_2
}


// Multiply two values to create one product.
fn multiply(val_1: f32, val_2: f32) -> f32{
    val_1 * val_2
}


// Divide one value by another.
fn divide(val_1: f32, val_2: f32) -> f32{
    val_1/val_2
}