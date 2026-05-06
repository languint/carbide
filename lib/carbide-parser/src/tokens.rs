use logos::Logos;

#[derive(Debug, PartialEq, Eq, Logos)]
pub enum Token<'s> {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'s str),
    #[regex(r"[0-9]+")]
    Number(&'s str),
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("%")]
    Percent,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("=")]
    Eq,
    #[token("==")]
    EqEq,
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEq,

    #[token("if")]
    If,
    #[token("else")]
    Else,

    #[token("&&")]
    And,
    #[token("||")]
    Or,

    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,

    #[token("<=")]
    LEq,
    #[token(">=")]
    GEq,
}

impl<'s> Token<'s> {
    /// Lexes the input string into a vector of tokens, throws out invalid tokens.
    #[must_use]
    pub fn lex_infallible(input: &'s str) -> Vec<Token<'s>> {
        Token::lexer(input).filter_map(Result::ok).collect()
    }

    /// Lexes the input string into a vector of tokens, returns errors for invalid tokens.
    #[must_use]
    pub fn lex_with_errors(input: &'s str) -> Vec<Result<Token<'s>, <Self as Logos<'s>>::Error>> {
        Token::lexer(input).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_infallible() {
        let input = "hello 123 + - * / ( ) { }";
        let expected = vec![
            Token::Identifier("hello"),
            Token::Number("123"),
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
        ];
        assert_eq!(Token::lex_infallible(input), expected);
    }

    #[test]
    fn test_lex_with_errors() {
        let input = "hello 123 + - * / ( ) { }";
        let expected = vec![
            Ok(Token::Identifier("hello")),
            Ok(Token::Number("123")),
            Ok(Token::Plus),
            Ok(Token::Minus),
            Ok(Token::Star),
            Ok(Token::Slash),
            Ok(Token::LParen),
            Ok(Token::RParen),
            Ok(Token::LBrace),
            Ok(Token::RBrace),
        ];
        assert_eq!(Token::lex_with_errors(input), expected);
    }
}
