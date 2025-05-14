mod token;
mod token_parse;
mod tokenizer;

use std::{env, fs, io::Write, process::Command};
use token::Token;
use token_parse::TokenParser;
use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Invalid number of arguments.");
        eprintln!("Usage: {} [options] <input_file>", args[0]);
        eprintln!("For more information, use: {} -h", args[0]);
        std::process::exit(1);
    }
    let mut output_file_name = "output";
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" => print_help(&args[0]),
            "--help" => print_help(&args[0]),
            "-o" => {
                i += 1;
                if i == (args.len() - 1) {
                    eprintln!("Too few arguments given for option: -o");
                    std::process::exit(1);
                }
                output_file_name = &args[i]
            }
            _ => {}
        }
        i += 1;
    }
    let input_file: &String = &args[args.len() - 1];
    println!("{input_file}");
    let contents = fs::read_to_string(input_file).expect("Error reading file.");
    println!("{contents}");
    let initial_tokens = lexical_parse_first_pass(contents);
    println!("{:?}", initial_tokens);
    let output = lexical_parse_second_pass(initial_tokens);
    print!("{}", output);
    let path = output_file_name.to_owned() + ".asm";
    let mut output_file = fs::File::create(path).expect("Failed to create output file.");
    write!(output_file, "{}", output).expect("Failed to write to output file.");
    Command::new("nasm")
        .arg("-felf64")
        .arg(output_file_name.to_owned() + ".asm")
        .output()
        .expect("Failed to compile assembly code.");
    Command::new("ld")
        .arg("-o")
        .arg(output_file_name)
        .arg(output_file_name.to_owned() + ".o")
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

fn print_help(file_path: &String) {
    println!("Usage: {} [options] <input_file>", file_path);
    println!("Options:");
    println!("  -h, --help                 Show this message");
    println!("  -o <output_file_name>      Use a specified file name for output");
    std::process::exit(0);
}
