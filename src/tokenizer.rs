use crate::token::Token;

#[derive(Debug)]
pub struct Tokenizer {
    input: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            tokens: Vec::new(),
        }
    }
    pub fn tokenize(mut self) -> Vec<Token> {
        println!("{:?}", self.input.chars());
        let mut i = 0;
        while i < self.input.len() {
            let mut current_token_value = String::new();
            let mut current_token_type = String::new();
            let c = self.input.chars().nth(i).unwrap();
            if c == '\n' {
                current_token_value = String::from("NEWLINE");
                current_token_type = String::from("NEWLINE");
            } else if c.is_alphabetic() {
                current_token_value.push(c);
                while self.input.chars().nth(i + 1).unwrap().is_alphabetic() {
                    current_token_value.push(self.input.chars().nth(i + 1).unwrap());
                    i += 1;
                }
                current_token_type = String::from("IDENTIFIER");
            } else if c.is_numeric() {
                current_token_value.push(c);
                current_token_type = String::from("INTEGER");
            } else if c == ';' {
                current_token_value = String::from(";");
                current_token_type = String::from("SEMICOLON");
            } else if c == '"' {
                while self.input.chars().nth(i + 1).unwrap() != '"' {
                    current_token_value.push(self.input.chars().nth(i + 1).unwrap());
                    println!("{:?}", self.input.chars().nth(i + 1).unwrap());
                    i += 1;
                }
                i += 1;
                current_token_type = String::from("STRING");
            }
            let current_token = Token {
                token_type: current_token_type,
                value: current_token_value,
            };
            println!("{:?}", current_token);
            self.tokens.push(current_token);
            i += 1;
        }
        self.tokens
    }
}
