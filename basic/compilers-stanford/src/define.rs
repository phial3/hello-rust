use std::fmt::{Display, Formatter};

pub const CODE: &str = "
var b3 == 2 * 3 + 2 * 4;
if(a == 3){

}else if(a == 4){

}else{

}
";

#[derive(Debug, Copy, Clone)]
pub enum KeyWord {
    VAR,
    // RETURN,
    IF,
}

impl KeyWord {
    pub fn call(self) -> &'static str {
        match self {
            KeyWord::VAR => "var",
            // KeyWord::RETURN => "return",
            KeyWord::IF => "if",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenType {
    NUM,
    ID,
    MINUS,
    ASTERISK,
    PLUS,
    SLASH,
    OPERATOR,
    KEYWORDS,
    LPAREN,
    RPAREN,
    LBRACES,
    RBRACES,
    SEMICOLON,
    WHITESPACE,
    ASSIGN,
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TokenType {
    pub fn precedence(self) -> i32 {
        match self {
            TokenType::NUM => 0,
            TokenType::ID => 0,
            TokenType::KEYWORDS => 0,
            TokenType::LPAREN => 3,
            TokenType::RPAREN => 3,
            TokenType::SEMICOLON => 0,
            TokenType::WHITESPACE => 0,
            TokenType::ASSIGN => 0,
            TokenType::PLUS => 1,
            TokenType::MINUS => 0,
            TokenType::ASTERISK => 2,
            TokenType::SLASH => 2,
            TokenType::EOF => 0,
            TokenType::LBRACES => 4,
            TokenType::RBRACES => 4,
            TokenType::OPERATOR => 0
        }
    }
    pub fn call(self) -> &'static str {
        match self {
            TokenType::NUM => "None",
            TokenType::ID => "ID",
            TokenType::KEYWORDS => "KEYWORDS",
            TokenType::LPAREN => "LPAREN",
            TokenType::RPAREN => "RPAREN",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::WHITESPACE => "WHITESPACE",
            TokenType::ASSIGN => "ASSIGN",
            TokenType::PLUS => "PLUS",
            TokenType::MINUS => "MINUS",
            TokenType::ASTERISK => "ASTERISK",
            TokenType::SLASH => "SLASH",
            TokenType::EOF => "EOF",
            TokenType::LBRACES => "LBRACES",
            TokenType::RBRACES => "RBRACES",
            TokenType::OPERATOR => "OPERATOR"
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NType {
    None,
    Int,
    Float,
    Strings,
}
