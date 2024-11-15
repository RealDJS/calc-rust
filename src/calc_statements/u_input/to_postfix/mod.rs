pub mod to_postfix {
    use std::collections::VecDeque;

    /// Establishes the order or operations.
    fn precedence(op: char) -> u8 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            '^' => 3,
            _ => 0, //Invalid operators
        }
    }

    /// Checks if a valid operator.
    fn is_operator(c: char) -> bool {
        "+-*/^".contains(c)
    }

    /// Converts an expression to postfix notation.
    pub fn infix_to_postfix(expression: &str) -> Result<Vec<String>, String> {
        let mut output: VecDeque<String> = VecDeque::with_capacity(0); // New List.
        let mut num: Vec<char> = Vec::new();
        let mut operators: Vec<char> = Vec::new(); // Valid operators.
        let mut parentheses: Vec<char> = Vec::new(); // Tracks Parentheses,

        for item in expression.chars() {
            match item {
                c if c.is_numeric() => output.push_back((c.to_string())),

                c if is_operator(c) => {
                    while operators
                        .last()
                        .map_or(false, |&top| precedence(top) >= precedence(c))
                    {
                        if let Some(op) = operators.pop() {
                            output.push_back(op.to_string());
                        }
                    }
                    operators.push(c);
                }

                '(' => parentheses.push(item),

                ')' => {
                    let mut parenthesis_resolved = false;
                    while let Some(top) = parentheses.pop() {
                        if top == '(' {
                            parenthesis_resolved = true;
                        }
                        //output.push_back(top.to_string());
                    }
                    if !parenthesis_resolved {
                        return Err("Mismatched parentheses.".to_string());
                    }
                }
                c if c.is_whitespace() => {}

                _ => return Err(format!("Invalid character in expression: {}", item)),
            }
        }

        // Pop remaining operators.
        if operators.iter().any(|&op| op == '(' || op == ')') {
            return Err("Mismatched parentheses".to_string());
        }

        while let Some(op) = operators.pop() {
            output.push_back(op.to_string());
        }
        Ok(output.into_iter().collect::<Vec<_>>())
    }
}
