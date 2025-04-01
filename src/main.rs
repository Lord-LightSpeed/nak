mod token;
mod token_parse;
mod tokenizer;

use std::{
    env, fs,
    fs::File,
    io::Write,
    process::{Command, Output},
};
use token::Token;
use token_parse::TokenParser;
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
    println!("{:?}", initial_tokens);
    let output = lexical_parse_second_pass(initial_tokens);
    print!("{}", output);
    let path = "output.asm";
    let mut output_file = fs::File::create(path).expect("Failed to create output file.");
    write!(output_file, "{}", output).expect("Failed to write to output file.");
    Command::new("nasm")
        .arg("-felf64")
        .arg("output.asm")
        .output()
        .expect("Failed to compile assembly code.");
    Command::new("ld")
        .arg("-o")
        .arg("output")
        .arg("output.o")
        .output()
        .expect("Failed to link object file.");
}
fn lexical_parse_first_pass(input: String) -> Vec<Token> {
    let tokenizer = Tokenizer::new(input);
    tokenizer.tokenize()
}
fn lexical_parse_second_pass(input: Vec<Token>) -> String {
    let token_parse = TokenParser::new(input);
    token_parse.parse()
}
