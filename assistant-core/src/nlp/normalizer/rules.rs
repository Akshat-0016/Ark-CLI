use crate::nlp::tokenizer::token::Token;

pub fn lowercase(token: &mut Token) {
    let lower = token.lexeme().to_lowercase();
    *token.lexeme_mut() = lower;
}
