use std::env;

mod concepts1;
mod concepts2;

fn input_from_args() -> String {
    let args: Vec<String> = env::args().collect();

    let mut input = String::new();

    if args.len() > 1 { 
        input = String::from(&args[1]);
    }

    input
}

fn parse_int(string_number: String) -> i32 {
    let result_number : i32 = match string_number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    result_number
}

fn main() {
    let input_string: String = input_from_args();
    let concept_number: i32 = parse_int(input_string);

    match concept_number {
        1 => {
            concepts1::variables_and_mutability::example();
            concepts1::data_types::example();
            concepts1::functions::example();
            concepts1::comments::example();
            concepts1::control_flow::example();
        },
        2 => {
            concepts2::ownership::example();
        },
        _ => println!("\nPass a number argument on `cargo run`.\nExample: `cargo run 1`\n"),
    }
}