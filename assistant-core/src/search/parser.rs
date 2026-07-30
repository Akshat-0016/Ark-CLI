use super::{ast::QueryNode, parser_error::ParserError, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse_or(&mut self) -> Result<QueryNode, ParserError> {
        let mut node = self.parse_and()?;

        while self.match_token(Token::Or) {
            let rhs = self.parse_and()?;

            node = QueryNode::Or(Box::new(node), Box::new(rhs));
        }

        Ok(node)
    }

    fn parse_and(&mut self) -> Result<QueryNode, ParserError> {
        let mut node = self.parse_not()?;

        while self.match_token(Token::And) {
            let rhs = self.parse_not()?;

            node = QueryNode::And(Box::new(node), Box::new(rhs));
        }

        Ok(node)
    }

    fn parse_not(&mut self) -> Result<QueryNode, ParserError> {
        if self.match_token(Token::Not) {
            return Ok(QueryNode::Not(Box::new(self.parse_not()?)));
        }

        self.parse_primary()
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();

        if token != Token::EOF {
            self.current += 1;
        }

        token
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token::EOF)
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if *self.peek() == expected {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn consume(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.match_token(expected.clone()) {
            Ok(())
        } else {
            Err(ParserError::MissingClosingParenthesis)
        }
    }

    fn parse_primary(&mut self) -> Result<QueryNode, ParserError> {
        match self.advance() {
            Token::Term(t) => Ok(QueryNode::Term(t)),

            Token::Phrase(p) => Ok(QueryNode::Phrase(p)),

            Token::LParen => {
                let node = self.parse_or()?;

                self.consume(Token::RParen)?;

                Ok(node)
            }

            Token::EOF => Err(ParserError::UnexpectedEOF),

            token => Err(ParserError::UnexpectedToken(format!("{:?}", token))),
        }
    }

    pub fn parse(&mut self) -> Result<QueryNode, ParserError> {
        let node = self.parse_or()?;

        if *self.peek() != Token::EOF {
            return Err(ParserError::UnexpectedToken(format!("{:?}", self.peek())));
        }

        Ok(node)
    }
}
