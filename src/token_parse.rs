use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug)]
pub struct TokenParser {
    input: Vec<Token>,
    output: String,
    i: usize,
}

impl TokenParser {
    pub fn new(input: Vec<Token>) -> TokenParser {
        TokenParser {
            input,
            output: String::from("section .text\n    global _start\n\n_start:\n"),
            i: 0,
        }
    }
    pub fn parse(mut self) -> String {
        while self.i < self.input.len() {
            match self.input[self.i].token_type {
                TokenType::Exit => self.parse_exit(),
                _ => self.output.push_str(""),
            }
            self.i += 1;
        }
        self.output
    }
    fn parse_exit(&mut self) {
        if self.i + 2 < self.input.len() {
            if self.input[self.i + 1].token_type == TokenType::Integer
            && self.input[self.i + 2].token_type == TokenType::Delimiter
            {
                self.output.push_str("    mov rax, 60\n    mov rdi, ");
                self.output.push_str(self.input[self.i + 1].value.as_str());
                self.output.push_str("\n    syscall\n");
            }
            self.i += 2;
        } else {
            eprintln!(
                "Error on token: {:?} invalid use of \"exit\" keyword.",
                self.i
            );
            std::process::exit(-1);
        }
    }
}
