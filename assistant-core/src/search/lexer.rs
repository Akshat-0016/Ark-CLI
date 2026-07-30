use super::token::Token;

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens.push(Token::EOF);

        let raw = tokens;
        let mut normalized = Vec::new();

        for token in raw {
            if let Some(prev) = normalized.last() {
                if Self::needs_default_operator(prev, &token) {
                    normalized.push(Token::Or);
                }
            }

            normalized.push(token);
        }

        normalized.push(Token::EOF);

        normalized
    }
    fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' => {
                    self.chars.next();
                }

                '(' => {
                    self.chars.next();
                    return Some(Token::LParen);
                }

                ')' => {
                    self.chars.next();
                    return Some(Token::RParen);
                }

                '"' => return Some(self.read_phrase()),

                _ => return Some(self.read_word()),
            }
        }

        None
    }

    fn read_word(&mut self) -> Token {
        let mut word = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() || c == '(' || c == ')' {
                break;
            }

            word.push(c);
            self.chars.next();
        }

        match word.to_lowercase().as_str() {
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Term(word),
        }
    }

    fn read_phrase(&mut self) -> Token {
        self.chars.next();

        let mut phrase = String::new();

        while let Some(c) = self.chars.next() {
            if c == '"' {
                break;
            }

            phrase.push(c);
        }

        Token::Phrase(phrase)
    }
    fn needs_default_operator(prev: &Token, next: &Token) -> bool {
        if matches!(
            (prev, next),
            (Token::Term(_), Token::Term(_))
                | (Token::Term(_), Token::Phrase(_))
                | (Token::Phrase(_), Token::Term(_))
                | (Token::Phrase(_), Token::Phrase(_))
                | (Token::RParen, Token::Term(_))
                | (Token::RParen, Token::Phrase(_))
                | (Token::Term(_), Token::LParen)
                | (Token::Phrase(_), Token::LParen)
        ) {
            true
        } else {
            false
        }
    }
}
