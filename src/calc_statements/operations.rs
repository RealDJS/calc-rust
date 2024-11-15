pub mod operations {
    /// Return the result for the given operation and two values.
    pub fn get_operation_result(val_1: f32, opr: char, val_2: f32) -> f32 {
        match opr {
            '*' => val_1 * val_2,
            '/' => val_1 / val_2,
            '+' => val_1 + val_2,
            '-' => val_1 - val_2,
            _ => {
                println!("Invalid operation.");
                return 0.0;
            }
        }
    }
}
