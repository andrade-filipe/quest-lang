use chumsky::prelude::*;
use crate::lexer::token::Token;
use crate::parser::ast::{Statement, Command, Expression, BinaryOp};

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>> + Clone {
    // Parser de expressões
    let expr = recursive(|expr| {
        let term = select! {
            Token::Number(n) => Expression::Number(n),
            Token::Identifier(id) => Expression::Identifier(id),
        };

        let op = just(Token::Plus).to(BinaryOp::Add)
            .or(just(Token::Minus).to(BinaryOp::Sub));

        term.clone()
            .then(op.then(term).repeated()) // repete (op term)
            .foldl(|lhs, (op, rhs)| Expression::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            })
    });

    // Parser de statements (recursivo)
    let statement = recursive(|statement| {
        // if ( expr ) { statement } else { statement }
        let if_parser = just(Token::If)
            .ignore_then(just(Token::LParen))
            .ignore_then(expr.clone())
            .then_ignore(just(Token::RParen))
            .then_ignore(just(Token::LBrace))
            .then(statement.clone())
            .then_ignore(just(Token::RBrace))
            .then(
                just(Token::Else)
                    .ignore_then(just(Token::LBrace))
                    .ignore_then(statement.clone())
                    .then_ignore(just(Token::RBrace))
            )
            .map(|(cond_then, else_branch)| {
                let (cond, then_branch) = cond_then;
                Statement::If {
                    condition: cond,
                    then_branch: Box::new(then_branch),
                    else_branch: Box::new(else_branch),
                }
            });

        // while ( expr ) { statement }
        let while_parser = just(Token::While)
            .ignore_then(just(Token::LParen))
            .ignore_then(expr.clone())
            .then_ignore(just(Token::RParen))
            .then_ignore(just(Token::LBrace))
            .then(statement.clone())
            .then_ignore(just(Token::RBrace))
            .map(|(cond, body)| Statement::While {
                condition: cond,
                body: Box::new(body),
            });

        // for ( expr ; expr ; expr ) { statement }
        let for_parser = just(Token::For)
            .ignore_then(just(Token::LParen))
            .ignore_then(expr.clone())
            .then_ignore(just(Token::Semicolon))
            .then(expr.clone())
            .then_ignore(just(Token::Semicolon))
            .then(expr.clone())
            .then_ignore(just(Token::RParen))
            .then_ignore(just(Token::LBrace))
            .then(statement.clone())
            .then_ignore(just(Token::RBrace))
            .map(|(((init, cond), update), body)| Statement::For {
                init,
                condition: cond,
                update,
                body: Box::new(body),
            });

        // Comandos simples
        let command_parser = choice((
            just(Token::MoveUp).to(Command::MoveUp),
            just(Token::MoveDown).to(Command::MoveDown),
            just(Token::MoveLeft).to(Command::MoveLeft),
            just(Token::MoveRight).to(Command::MoveRight),
            just(Token::Jump).to(Command::Jump),
            just(Token::Attack).to(Command::Attack),
            just(Token::Defend).to(Command::Defend),
        ))
        .map(Statement::Command);

        choice((if_parser, while_parser, for_parser, command_parser))
    });

    // Parser final: zero ou mais statements
    statement.repeated()
}
