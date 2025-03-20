mod lexer;
mod parser;

use lexer::lexer_impl::lex;
use parser::parser_impl::parse_tokens;

fn main() {
    // Exemplo de código em QuestLang
    let code = r#"
        if (hero + 10 { move_up } else { jump }
        while (enemy - 5) { attack }
    "#;

    // Executa o lexer (Logos) para gerar um vetor de tokens
    let tokens = lex(code);
    println!("Tokens gerados:");
    for token in &tokens {
        println!("{:?}", token);
    }

    // Executa o parser (Chumsky) usando os tokens gerados
    match parse_tokens(tokens) {
        Ok(ast) => {
            println!("\nAST gerada com sucesso:");
            println!("{:#?}", ast);
        }
        Err(errors) => {
            println!("\nErros de parsing:");
            for error in errors {
                println!("{:?}", error);
            }
        }
    }
}
