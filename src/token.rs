#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}
impl Token {}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Identifier,
    Integer,
    String,
    Delimiter,
    Operator,
    Exit,
    Newline,
}
