use std::env;
use std::fs;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide an input file as first argument");
        return;
    }

    let input_text = fs::read_to_string(&args[1]).expect("could not read file");
    let tokens = parse_tokens(input_text.to_string());
    let operations = tokens_to_operations(tokens);
    let memory = run_operations(operations);

    println!("");
    println!("");
    println!("Memory: {:?}", memory);
}

fn run_operations(operations: Vec<Operations>) -> Vec<i32> {
    let mut memory_pointer: i32 = 0;
    let mut stack_pointer = 0;
    let mut memory: Vec<i32> = vec![];
    let max_pointer = operations.len();
    while stack_pointer < max_pointer {
        while memory.len() <= (memory_pointer as usize) {
            memory.push(0);
        }
        match operations[stack_pointer] {
            Operations::Shift(val) => {
                memory_pointer = (memory_pointer + (val)).min(100000).max(0);
            }
            Operations::ChangeValue(val) => {
                memory[memory_pointer as usize] = memory[memory_pointer as usize] + (val as i32);
            }
            Operations::Output => {
                print!("{}", ((memory[memory_pointer as usize] as u8) as char));
            }
            Operations::Input => {
                let input_option: Option<u8> = std::io::stdin()
                    .bytes()
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u8);
                if let Some(input_byte) = input_option {
                    memory[memory_pointer as usize] = input_byte as i32;
                }
            }
            Operations::Jump(index, jump_is_left) => {
                let mem_val = memory[memory_pointer as usize];
                if jump_is_left && mem_val == 0 || !jump_is_left && mem_val != 0 {
                    stack_pointer = index as usize;
                }
            }
        }
        stack_pointer = stack_pointer + 1;
    }
    memory
}

fn tokens_to_operations(tokens: Vec<Tokens>) -> Vec<Operations> {
    let mut res: Vec<Operations> = vec![];
    let mut left_bracket_indexes: Vec<i32> = vec![];
    for token in tokens {
        if res.len() == 0 {
            //first token
            res.push(token_to_operation(&token));
        } else {
            let prev_index = res.len() - 1;
            //atleast one operation in result
            if let Some(optimized_operation) = get_optimized_operation(&res[prev_index], &token) {
                res[prev_index] = optimized_operation;
            } else {
                let mut new_operation = token_to_operation(&token);
                match token {
                    Tokens::LeftJump => {
                        left_bracket_indexes.push(prev_index as i32 + 1);
                    }
                    Tokens::RightJump => {
                        let matching_left_index =
                            left_bracket_indexes.pop().expect("Found a ] before any [");
                        new_operation = Operations::Jump(matching_left_index, false);
                        res[matching_left_index as usize] =
                            Operations::Jump(prev_index as i32 + 1, true);
                    }
                    _ => {}
                }
                res.push(new_operation);
            }
        }
    }
    res
}

fn token_to_operation(token: &Tokens) -> Operations {
    match token {
        Tokens::ShiftRight => Operations::Shift(1),
        Tokens::ShiftLeft => Operations::Shift(-1),
        Tokens::Increment => Operations::ChangeValue(1),
        Tokens::Decrement => Operations::ChangeValue(-1),
        Tokens::Output => Operations::Output,
        Tokens::Input => Operations::Input,
        Tokens::LeftJump => Operations::Jump(0, true),
        Tokens::RightJump => Operations::Jump(0, false),
    }
}

fn get_optimized_operation(operation: &Operations, next_token: &Tokens) -> Option<Operations> {
    match (operation, next_token) {
        (Operations::Shift(val), Tokens::ShiftRight) => Some(Operations::Shift(val + 1)),
        (Operations::Shift(val), Tokens::ShiftLeft) => Some(Operations::Shift(val + -1)),
        (Operations::ChangeValue(val), Tokens::Increment) => Some(Operations::ChangeValue(val + 1)),
        (Operations::ChangeValue(val), Tokens::Decrement) => Some(Operations::ChangeValue(val - 1)),
        _ => None,
    }
}

fn parse_tokens(input: String) -> Vec<Tokens> {
    let mut res: Vec<Tokens> = vec![];
    let chars: Vec<char> = input.chars().collect();
    for cur_char in chars {
        if let Some(operation) = character_to_token(cur_char) {
            res.push(operation);
        }
    }
    res
}

fn character_to_token(input: char) -> Option<Tokens> {
    match input {
        '>' => Some(Tokens::ShiftRight),
        '<' => Some(Tokens::ShiftLeft),
        '+' => Some(Tokens::Increment),
        '-' => Some(Tokens::Decrement),
        '.' => Some(Tokens::Output),
        ',' => Some(Tokens::Input),
        '[' => Some(Tokens::LeftJump),
        ']' => Some(Tokens::RightJump),
        _ => None,
    }
}

#[derive(Debug)]
enum Tokens {
    ShiftRight,
    ShiftLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LeftJump,
    RightJump,
}
#[derive(Debug)]
enum Operations {
    Shift(i32),
    ChangeValue(i32),
    Output,
    Input,
    Jump(i32, bool),
}
