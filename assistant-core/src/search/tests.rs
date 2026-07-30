use crate::search::{lexer::Lexer, parser::Parser};

#[test]
fn parse_simple_query() {
    let mut lexer = Lexer::new("rust AND esp32");

    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    assert!(ast.is_ok());
}
