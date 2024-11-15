pub mod u_input {
    use std::io;

    pub fn get_operation_set() -> Vec<char> {
        let input = get_input();

        let list = get_chars(input);

        list
    }

    /// Get expression from the user.
    fn get_input() -> String {
        let mut input: String = String::new();
        while input.trim().is_empty() {
            println!("Enter an expression: ");
            input.clear();

            if io::stdin().read_line(&mut input).is_ok() {
                if !input.trim().is_empty() {
                    return input;
                } else {
                    println!("Input is empty. Try again.");
                }
            } else {
                println!("Failed to read line. Please try again.");
                input.clear();
            }
        }
        input
    }

    /// Parse input to make a list.
    fn get_chars(input: String) -> Vec<char> {
        let mut characters: Vec<char> = Vec::new();

        for ch in input.chars() {
            if ch != ' ' && ch != '\r' && ch != '\n' {
                characters.push(ch);
            }
        }
        characters
    }
}
