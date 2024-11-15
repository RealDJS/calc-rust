mod to_postfix;
pub mod u_input {
    use std::io;

    use super::to_postfix::to_postfix::infix_to_postfix;

    pub fn get_operation_set() -> Vec<String> {
        let input = get_input();

        match infix_to_postfix(&input) {
            Ok(list) => list,
            Err(e) => {
                println!("Error: {}", e);
                Vec::new()
            }
        }
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
    fn get_chars(u_input: String) -> Vec<String> {
        let characters: Vec<char> = vec![
            '(', ')', '[', ']', '{', '}', '*', '^', '/', '+', '-', '%', 'x',
        ];
        let mut items: Vec<String> = Vec::new();
        let mut curr_num: String = String::new(); // Current number in list.

        for ch in u_input.chars() {
            if ch.is_digit(10) {
                curr_num.push(ch);
            } else {
                if !curr_num.is_empty() {
                    items.push(curr_num.clone()); // Push complete number to list.
                    curr_num.clear();
                }
                // Pushes other items to list.
                if characters.contains(&ch) {
                    items.push(ch.to_string());
                }
            }
        }
        items
    }
}
