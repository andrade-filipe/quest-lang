use crate::lexer::token::Token;
use crate::parser::ast::*;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    /// Parseia o programa inteiro, retornando uma lista de statements
    pub fn parse_program(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();
        while self.tokens.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    /// Parseia um statement, que pode ser um comando ou uma estrutura de controle
    fn parse_statement(&mut self) -> Result<Statement, String> {
        if let Some(token) = self.tokens.peek() {
            match token {
                Token::If => self.parse_if_stmt(),
                Token::While => self.parse_while_stmt(),
                Token::For => self.parse_for_stmt(),
                Token::LBrace => self.parse_block(),
                // Caso não seja nenhum dos casos acima, assume comando
                _ => self.parse_command(),
            }
        } else {
            Err("Fim inesperado da entrada ao parsear statement".to_string())
        }
    }

    /// Parseia um bloco: { statement* }
    fn parse_block(&mut self) -> Result<Statement, String> {
        self.consume(Token::LBrace)?;
        let mut stmts = Vec::new();
        while let Some(token) = self.tokens.peek() {
            if *token == Token::RBrace {
                break;
            }
            stmts.push(self.parse_statement()?);
        }
        self.consume(Token::RBrace)?;
        Ok(Statement::Block(stmts))
    }

    /// Parseia um comando simples (movimento ou ação)
    fn parse_command(&mut self) -> Result<Statement, String> {
        if let Some(token) = self.tokens.next() {
            let cmd = match token {
                Token::MoveUp => Command::MoveUp,
                Token::MoveDown => Command::MoveDown,
                Token::MoveLeft => Command::MoveLeft,
                Token::MoveRight => Command::MoveRight,
                Token::Jump => Command::Jump,
                Token::Attack => Command::Attack,
                Token::Defend => Command::Defend,
                _ => return Err(format!("Token inesperado em comando: {:?}", token)),
            };
            Ok(Statement::Command(cmd))
        } else {
            Err("Fim inesperado da entrada ao parsear comando".to_string())
        }
    }

    /// Parseia um if: if ( expr ) { statement } else { statement }
    fn parse_if_stmt(&mut self) -> Result<Statement, String> {
        self.consume(Token::If)?;
        self.consume(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.consume(Token::RParen)?;
        let then_branch = Box::new(self.parse_statement()?);
        self.consume(Token::Else)?;
        let else_branch = Box::new(self.parse_statement()?);
        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parseia um while: while ( expr ) { statement }
    fn parse_while_stmt(&mut self) -> Result<Statement, String> {
        self.consume(Token::While)?;
        self.consume(Token::LParen)?;
        let condition = self.parse_expression()?;
        self.consume(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(Statement::While { condition, body })
    }

    /// Parseia um for: for ( expr ; expr ; expr ) { statement }
    fn parse_for_stmt(&mut self) -> Result<Statement, String> {
        self.consume(Token::For)?;
        self.consume(Token::LParen)?;
        let init = self.parse_expression()?;
        self.consume(Token::Semicolon)?;
        let condition = self.parse_expression()?;
        self.consume(Token::Semicolon)?;
        let update = self.parse_expression()?;
        self.consume(Token::RParen)?;
        let body = Box::new(self.parse_statement()?);
        Ok(Statement::For {
            init,
            condition,
            update,
            body,
        })
    }

    /// Parseia uma expressão aritmética simples (identificador, número ou parênteses).
    /// Suporta a operação binária de adição e subtração.
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;
        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = match self.tokens.next().unwrap() {
                        Token::Plus => BinaryOp::Plus,
                        Token::Minus => BinaryOp::Minus,
                        _ => unreachable!(),
                    };
                    let right = self.parse_term()?;
                    expr = Expression::Binary {
                        left: Box::new(expr),
                        op,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    /// Parseia um termo: número, identificador ou expressão entre parênteses.
    fn parse_term(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.next() {
            match token {
                Token::Number(n) => Ok(Expression::Number(n)),
                Token::Identifier(id) => Ok(Expression::Identifier(id)),
                Token::LParen => {
                    let expr = self.parse_expression()?;
                    self.consume(Token::RParen)?;
                    Ok(expr)
                }
                _ => Err(format!("Token inesperado em expressão: {:?}", token)),
            }
        } else {
            Err("Fim inesperado da entrada ao parsear expressão".to_string())
        }
    }

    /// Função auxiliar para consumir um token esperado.
    fn consume(&mut self, expected: Token) -> Result<(), String>
    where
        Token: PartialEq,
    {
        if let Some(token) = self.tokens.next() {
            if token == expected {
                Ok(())
            } else {
                Err(format!("Esperado token {:?}, mas encontrado {:?}", expected, token))
            }
        } else {
            Err(format!("Esperado token {:?}, mas chegou ao fim da entrada", expected))
        }
    }
}
