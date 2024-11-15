pub mod operations {
    /// Return the result for the given operation and two values.
    pub fn get_operation_result(val_1: f32, opr: String, val_2: f32) -> f32 {
        match opr.as_str() {
            "*" => multiply(val_1, val_2),
            "/" => divide(val_1, val_2),
            "+" => add(val_1, val_2),
            "-" => subtract(val_1, val_2),
            _ => {
                println!("Invalid operation.");
                return 0.0;
            }
        }
    }

    /// Add two values to create one sum.
    fn add(val_1: f32, val_2: f32) -> f32 {
        val_1 + val_2
    }

    /// Subtract val_2 from val_1.
    fn subtract(val_1: f32, val_2: f32) -> f32 {
        val_1 - val_2
    }

    /// Multiply two values to create one product.
    fn multiply(val_1: f32, val_2: f32) -> f32 {
        val_1 * val_2
    }

    /// Divide one value by another.
    fn divide(val_1: f32, val_2: f32) -> f32 {
        val_1 / val_2
    }
}
