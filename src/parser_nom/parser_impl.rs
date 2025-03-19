/* 
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};
use crate::parser::ast::*;

pub fn parse_program(input: &str) -> IResult<&str, Vec<Statement>> {
    // many0 retorna um parser. Para usá-lo diretamente,
    // chamamos (input) no final, assim:
    many0(delimited(multispace0, parse_statement, multispace0))(input)
}

// ---------------------------------------------
// Funções de Statement
// ---------------------------------------------
fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_if,
        parse_while,
        parse_for,
        parse_block,
        parse_command
    ))(input)
}

/// Bloco: { statement* }
fn parse_block(input: &str) -> IResult<&str, Statement> {
    let (input, stmts) = delimited(
        char('{'),
        many0(delimited(multispace0, parse_statement, multispace0)),
        char('}')
    )(input)?;
    Ok((input, Statement::Block(stmts)))
}

/// Comando simples: move_up, move_down, ...
fn parse_command(input: &str) -> IResult<&str, Statement> {
    let (input, cmd) = alt((
        map(tag("move_up"), |_| Command::MoveUp),
        map(tag("move_down"), |_| Command::MoveDown),
        map(tag("move_left"), |_| Command::MoveLeft),
        map(tag("move_right"), |_| Command::MoveRight),
        map(tag("jump"), |_| Command::Jump),
        map(tag("attack"), |_| Command::Attack),
        map(tag("defend"), |_| Command::Defend),
    ))(input)?;
    Ok((input, Statement::Command(cmd)))
}

/// if ( expr ) { statement } else { statement }
fn parse_if(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, condition) = delimited(
        char('('),
        preceded(multispace0, parse_expression),
        preceded(multispace0, char(')'))
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, then_branch) = parse_block(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("else")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, else_branch) = parse_block(input)?;
    Ok((input, Statement::If {
        condition,
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    }))
}

/// while ( expr ) { statement }
fn parse_while(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("while")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, condition) = delimited(
        char('('),
        preceded(multispace0, parse_expression),
        preceded(multispace0, char(')'))
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = parse_block(input)?;
    Ok((input, Statement::While {
        condition,
        body: Box::new(body),
    }))
}

/// for ( expr ; expr ; expr ) { statement }
fn parse_for(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, init) = delimited(multispace0, parse_expression, multispace0)(input)?;
    let (input, _) = char(';')(input)?;
    let (input, condition) = delimited(multispace0, parse_expression, multispace0)(input)?;
    let (input, _) = char(';')(input)?;
    let (input, update) = delimited(multispace0, parse_expression, multispace0)(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = parse_block(input)?;
    Ok((input, Statement::For {
        init,
        condition,
        update,
        body: Box::new(body),
    }))
}

// ---------------------------------------------
// Funções de Expression
// ---------------------------------------------

fn parse_expression(input: &str) -> IResult<&str, Expression> {
    let (input, left) = parse_term(input)?;
    // many0 de pares (op, termo)
    let (input, exprs) = many0((
        preceded(multispace0, alt((tag("+"), tag("-")))),
        preceded(multispace0, parse_term),
    ))(input)?;

    let expr = exprs.into_iter().fold(left, |acc, (op, right)| {
        let op_enum = if op == "+" { BinaryOp::Plus } else { BinaryOp::Minus };
        Expression::Binary {
            left: Box::new(acc),
            op: op_enum,
            right: Box::new(right),
        }
    });
    Ok((input, expr))
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    alt((
        // Número
        map_res(digit1, |digits: &str| digits.parse::<i64>().map(Expression::Number)),
        // Identificador
        map(take_while1(|c: char| c.is_alphabetic()), |s: &str| {
            Expression::Identifier(s.to_string())
        }),
        // ( expr )
        delimited(char('('), parse_expression, char(')'))
    ))(input)
}

pub fn parse(tokens: Vec<String>) -> Result<(Vec<String>, String), String> {
    if tokens.is_empty() {
        Err("No tokens to parse".to_string())
    } else {
        Ok((vec![], format!("AST (Nom): {}", tokens.join(" "))))
    }
}
*/