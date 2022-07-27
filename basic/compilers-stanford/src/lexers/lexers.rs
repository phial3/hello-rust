use regex::Regex;

use crate::define::define::TokenType::{
    ASSIGN, ASTERISK, EOF, ID, KEYWORDS, LPAREN, MINUS, NUM,
    PLUS, RPAREN, SEMICOLON, SLASH, LBRACES, RBRACES,
};
use crate::{is_blank, is_digit, is_keywords, is_letter, is_new_line, is_valid_id, is_operator};
use crate::define::define::{CODE, TokenType};

#[derive(Debug, Eq, PartialEq)]
enum S {
    Start,
    Done,
    Num,
    Operator,
    ID,
}

#[derive(Debug)]
pub struct Token {
    pub line: i32,
    pub t_type: TokenType,
    pub literal: &'static str,
}

impl Token {
    fn form(cur_line: i32, _type: TokenType, _literal: &'static str) -> Self {
        Token {
            line: cur_line,
            t_type: _type,
            literal: _literal,
        }
    }
}

pub fn analysis() -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut state = S::Start;
    let mut index = 0;
    let mut lookup = 0;
    let mut cur_line = 0;
    let new_code = CODE.as_bytes();
    while lookup < new_code.len() {
        while state != S::Done {
            let tmp = String::from(new_code[lookup] as char);
            lookup += 1;
            let char = tmp.as_str();
            match state {
                S::Start => {
                    if is_digit!(char) {
                        state = S::Num;
                    } else if is_new_line!(char) {
                        state = S::Done;
                    } else if is_blank!(char) {
                        state = S::Done;
                    } else if is_letter!(char) {
                        state = S::ID;
                    } else if is_operator!(&char) {
                        state = S::Operator;
                    } else if char == ";" {
                        tokens.push(Token::form(cur_line, SEMICOLON, ";"));
                        cur_line += 1;
                        state = S::Done;
                    } else if char == "(" {
                        tokens.push(Token::form(cur_line, LPAREN, "("));
                        state = S::Done;
                    } else if char == ")" {
                        tokens.push(Token::form(cur_line, RPAREN, ")"));
                        state = S::Done;
                    } else if char == "{" {
                        tokens.push(Token::form(cur_line, LBRACES, "{"));
                        state = S::Done;
                    } else if char == "}" {
                        tokens.push(Token::form(cur_line, RBRACES, "}"));
                        state = S::Done;
                    };
                }
                S::Done => continue,
                S::Num => {
                    if is_digit!(char) {
                        state = S::Num;
                    } else {
                        tokens.push(Token::form(cur_line, NUM, &CODE.clone()[index..lookup - 1]));
                        lookup -= 1;
                        state = S::Done;
                    }
                }
                S::ID => {
                    if is_valid_id!(char) {
                        state = S::ID;
                    } else {
                        let temp_token = &CODE.clone()[index..lookup - 1];
                        lookup -= 1;
                        if is_keywords!(&temp_token) {
                            let key_word = temp_token;
                            tokens.push(Token::form(cur_line, KEYWORDS, key_word));
                        } else {
                            tokens.push(Token::form(cur_line, ID, temp_token));
                        }
                        state = S::Done;
                    }
                }
                S::Operator => {
                    if is_operator!(&char) {
                        state = S::Operator;
                    } else {
                        tokens.push(Token::form(cur_line, ID, &CODE.clone()[index..lookup - 1]));
                        lookup -= 1;
                        state = S::Done;
                    }
                }
            }
        }
        index = lookup;
        state = S::Start;
    }
    tokens.push(Token::form(cur_line, EOF, "EOF"));
    tokens
}
