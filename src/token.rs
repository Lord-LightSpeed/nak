#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
impl Token {
    pub fn new() -> Token {
        Token {
            token_type: TokenType::Identifier,
            value: String::from(""),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Integer,
    String,
    Delimiter,
    Exit,
    Newline,
}
