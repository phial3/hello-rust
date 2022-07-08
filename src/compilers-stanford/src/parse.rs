use crate::ast::{Literal, OperatorExpress, VarStatements, E, ID, S, IfStatement};
use crate::define::TokenType::EOF;
use crate::define::{KeyWord, NType, TokenType};
use crate::hashmap;
use crate::lexers::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct TokenScaner {
    scan_token: usize,
    curr_token: usize,
    tokens: Vec<Token>,
}

impl TokenScaner {
    pub(crate) fn new(_tokens: Vec<Token>) -> Option<TokenScaner> {
        let parse = TokenScaner {
            scan_token: 0,
            curr_token: 0,
            tokens: _tokens,
        };
        Some(parse)
    }
    pub(crate) fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.scan_token);
        self.scan_token += 1;
        token
    }
    pub(crate) fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.scan_token)
    }
    // pub(crate) fn end_statement(&mut self)  {
    //     self.curr_token  = self.scan_token;
    // }
}

pub struct Parser {
    statement_func_map: HashMap<&'static str, ParserStatementFunc>,
    express_func_map: HashMap<&'static str, ParserExpressFunc>,
    scaner: TokenScaner,
}

pub(crate) type ParserExpressFunc =
    fn(_: &mut Parser, leftE: Option<Box<dyn E>>) -> Option<Box<dyn E>>;
pub(crate) type ParserStatementFunc = fn(_: &mut Parser) -> Option<Box<dyn S>>;

impl Parser {
    pub(crate) fn new(scanner: TokenScaner) -> Self {
        Parser {
            statement_func_map: hashmap!(),
            express_func_map: hashmap!(),
            scaner: scanner,
        }
    }
    pub(crate) fn register_statement(&mut self, token_type: KeyWord, func: ParserStatementFunc) {
        self.statement_func_map.insert(token_type.call(), func);
    }
    pub(crate) fn register_express(&mut self, token_type: TokenType, func: ParserExpressFunc) {
        self.express_func_map.insert(token_type.call(), func);
    }

    pub(crate) fn get_express(&self, token_type: TokenType) -> Option<&ParserExpressFunc> {
        self.express_func_map.get(token_type.call())
    }

    pub(crate) fn get_statement(&mut self, key_word: &'static str) -> Option<&ParserStatementFunc> {
        self.statement_func_map.get(key_word)
    }

    pub(crate) fn exec(&mut self) {
        loop {
            let token = self.scaner.next_token().unwrap();
            if token.t_type == EOF {
                break;
            }
            let key_word = token.literal;
            let wrap_exec_func = self.get_statement(key_word);
            match wrap_exec_func {
                None => {
                    println!("{} Not Implement!", key_word)
                }
                Some(exec) => {
                    let e = exec(self);
                    println!("{}", e.unwrap());
                }
            }
        }
    }
}

pub(crate) fn func_parser_var(parser: &mut Parser) -> Option<Box<dyn S>> {
    let mut _statment = VarStatements {
        m_type: NType::Int,
        identifier: None,
        init: None,
    };
    _statment.identifier = func_parser_id(parser, None);
    let assign_token = parser.scaner.next_token().unwrap();
    if assign_token.t_type != TokenType::ASSIGN {
        panic!(
            "友情提示:行:{} 期望 '=' 找到 '{}'!",
            assign_token.line, assign_token.literal
        )
    }
    _statment.init = parser_express(parser, 0);
    Some(Box::new(_statment))
}

pub(crate) fn func_parser_if(parser: &mut Parser) -> Option<Box<dyn S>> {
        let mut _statment = IfStatement {
            test: None,
            alternate: None
        };
    None
    // _statment.identifier = func_parser_id(parser, None);
    // let assign_token = parser.scaner.next_token().unwrap();
    // if assign_token.t_type != TokenType::ASSIGN {
    //     panic!(
    //         "友情提示:行:{} 期望 '=' 找到 '{}'!",
    //         assign_token.line, assign_token.literal
    //     )
    // }
    // _statment.init = parser_express(parser, 0);
    // Some(Box::new(_statment))
}

pub(crate) fn parser_operator_express(
    parser: &mut Parser,
    left_e: Option<Box<dyn E>>,
) -> Option<Box<dyn E>> {
    let mut _express = OperatorExpress {
        left: left_e,
        operator: "".to_string(),
        right: None,
    };
    let token = parser.scaner.next_token().unwrap();
    let token_priority = token.t_type.precedence();
    _express.operator = String::from(token.literal);
    _express.right = parser_express(parser, token_priority);
    Some(Box::new(_express))
}

pub(crate) fn parser_express(parser: &mut Parser, precedence: i32) -> Option<Box<dyn E>> {
    let left_express = parser
        .get_express(parser.scaner.peek().unwrap().t_type)
        .unwrap();
    let mut left_t = left_express(parser, None);
    loop {
        match parser.scaner.peek() {
            None => break,
            Some(token) => {
                if token.t_type.precedence() <= precedence {
                    break;
                }
                let exec = parser.get_express(token.t_type).unwrap();
                left_t = exec(parser, left_t);
            }
        }
    }
    left_t
}

pub(crate) fn parser_semicolon(_: &mut Parser, left_e: Option<Box<dyn E>>) -> Option<Box<dyn E>> {
    left_e
}

pub(crate) fn parser_literal(parser: &mut Parser, _: Option<Box<dyn E>>) -> Option<Box<dyn E>> {
    let mut literal = Literal::new();
    let token = parser.scaner.next_token().unwrap();
    literal.value = String::from(token.literal);
    Some(Box::new(literal))
}

pub(crate) fn func_parser_id(parser: &mut Parser, _: Option<Box<dyn E>>) -> Option<Box<dyn E>> {
    let token = parser.scaner.next_token().unwrap();
    if token.t_type != TokenType::ID {
        panic!("友情提示:行:{} 变量名错误啦!", token.line)
    }
    let _express = ID {
        name: token.literal.parse().unwrap(),
        xtype: NType::None,
    };
    Some(Box::new(_express))
}

pub(crate) fn parser_lparen_express(parser: &mut Parser, _: Option<Box<dyn E>>) -> Option<Box<dyn E>> {
    let _express = parser_express(parser,0);
    if parser.scaner.next_token().unwrap().t_type != TokenType::RPAREN{
        panic!("友情提示:行: 期望 ')' 找到 '{}' !",parser.scaner.peek().unwrap().literal)
    }
    _express
}