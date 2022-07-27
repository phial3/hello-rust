use StanfordCompilers_priciples::lexers::lexers;
use StanfordCompilers_priciples::define::define::{KeyWord, TokenType};
use StanfordCompilers_priciples::parse::parse::{
    func_parser_id, func_parser_var, parser_literal,
    parser_operator_express, parser_semicolon, Parser,
    TokenScanner, parser_lparen_express, func_parser_if,
};

fn main() {
    let tokens = lexers::analysis();
    let token_scanner = TokenScanner::new(tokens).unwrap();
    let mut parser = Parser::new(token_scanner);

    parser.register_statement(KeyWord::VAR, func_parser_var);
    parser.register_statement(KeyWord::IF, func_parser_if);

    parser.register_express(TokenType::ID, func_parser_id);
    parser.register_express(TokenType::ASTERISK, parser_operator_express);
    parser.register_express(TokenType::PLUS, parser_operator_express);
    parser.register_express(TokenType::SLASH, parser_operator_express);
    parser.register_express(TokenType::NUM, parser_literal);
    parser.register_express(TokenType::SEMICOLON, parser_semicolon);
    parser.register_express(TokenType::LPAREN, parser_lparen_express);

    parser.exec();
}
