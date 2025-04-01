use crate::token::Token;
use crate::token::TokenType;

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
            let mut current_token_type = TokenType::Identifier;
            let c = self.input.chars().nth(i).unwrap();
            if c == '\n' {
                current_token_value = String::from(r"\n");
                current_token_type = TokenType::Newline;
            } else if c.is_alphabetic() {
                current_token_value.push(c);
                while self.input.chars().nth(i + 1).unwrap().is_alphabetic() {
                    current_token_value.push(self.input.chars().nth(i + 1).unwrap());
                    i += 1;
                }
                match current_token_value.as_str() {
                    "exit" => current_token_type = TokenType::Exit,
                    _ => current_token_type = TokenType::Identifier,
                }
            } else if c.is_numeric() {
                current_token_value.push(c);
                while self.input.chars().nth(i + 1).unwrap().is_numeric() {
                    current_token_value.push(self.input.chars().nth(i + 1).unwrap());
                    i += 1;
                }
                current_token_type = TokenType::Integer;
            } else if c == ';' {
                current_token_value = String::from(";");
                current_token_type = TokenType::Delimiter;
            } else if c == '"' {
                while self.input.chars().nth(i + 1).unwrap() != '"' {
                    current_token_value.push(self.input.chars().nth(i + 1).unwrap());
                    println!("{:?}", self.input.chars().nth(i + 1).unwrap());
                    i += 1;
                }
                i += 1;
                current_token_type = TokenType::String;
            } else if c == '+' || c == '-' || c == '*' || c == '/' {
                current_token_value.push(c);
                current_token_type = TokenType::Operator;
            }
            let current_token = Token {
                token_type: current_token_type,
                value: current_token_value,
            };
            if current_token.value.len() > 0 {
                println!("{:?}", current_token);
                self.tokens.push(current_token);
            }
            i += 1;
        }
        self.tokens
    }
}
