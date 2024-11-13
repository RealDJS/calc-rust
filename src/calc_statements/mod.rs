mod operations;
mod u_input;
pub mod calc_statements {
    use crate::calc_statements::{
        operations::operations::get_operation_result, u_input::u_input::get_operation_set,
    };

    pub fn calc_state() -> f32 {
        let list: Vec<char> = get_operation_set();

        let val_1: f32 = parse_num(list[0]);

        let opr: char = list[1];

        let val_2: f32 = parse_num(list[2]);

        let result: f32 = get_operation_result(val_1, opr, val_2);

        println!("{:?}", list);

        result
    }

    fn parse_num(number: char) -> f32 {
        let num = number;

        match num.to_string().parse::<f32>() {
            Ok(num) => return num,
            Err(e) => println!("Failed to parse :{}", e),
        }
        0.0
    }
}
