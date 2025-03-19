use crate::lexer::token::Token;
use logos::Logos;

pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Cria um novo Lexer a partir da string de entrada.
    pub fn new(input: &'a str) -> Self {
        Lexer {
            inner: Token::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|res| match res {
            Ok(token) => token,
            Err(_) => Token::Error,
        })
    }
}

pub fn lex(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut lexer = Token::lexer(input);
    while let Some(token) = lexer.next() {
        tokens.push(format!("{:?}", token));
    }
    tokens
}