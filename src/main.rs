mod token;
mod tokenizer;

use std::{env, fs};
use token::Token;
use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Invalid number of arguments.");
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_file: &String = &args[1];
    println!("{input_file}");
    let contents = fs::read_to_string(input_file).expect("Error reading file.");
    println!("{contents}");
    let initial_tokens = lexical_parse_first_pass(contents);
    print!("{:?}", initial_tokens);
}
fn lexical_parse_first_pass(input: String) -> Vec<Token> {
    let tokenizer = Tokenizer::new(input);
    tokenizer.tokenize()
}
