#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub value: String,
}
impl Token {
    pub fn new() -> Token {
        Token { token_type: String::from(""), value: String::from("") }
    }
}