mod lexer;
mod parser;

use chumsky::Parser;
use lexer::lexer_impl::Lexer;
use lexer::token::Token;
use parser::parser_impl::parser;

fn main() {
    let input = r#"
        if (hero + 10) { move_up } else { jump }
        while (enemy - 5) { attack }
    "#;

    // Gera os tokens a partir do wrapper Lexer (que mapeia os erros para Token::Error)
    let tokens: Vec<Token> = Lexer::new(input).collect();

    println!("Tokens:");
    for token in &tokens {
        println!("{:?}", token);
    }

    // Executa o parser sobre o vetor de tokens
    let ast = parser().parse(tokens);

    match ast {
        Ok(statements) => {
            println!("AST:");
            for stmt in statements {
                println!("{:#?}", stmt);
            }
        },
        Err(errors) => {
            eprintln!("Erros de parsing:");
            for err in errors {
                eprintln!("{:?}", err);
            }
        }
    }
}
