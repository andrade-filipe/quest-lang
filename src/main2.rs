use std::io::{self, Write};

// Importa explicitamente cada módulo (eles estão no mesmo crate)
mod lexer_manual;
mod lexer;
mod parser_manual;
mod parser_nom;
mod parser;

// Re-exporta as funções (opcional) ou chama diretamente com nome do módulo
use crate::lexer_manual::lexer_impl::lex as manual_lex;
use crate::lexer::lexer_impl::lex as logos_lex;
use crate::parser_manual::parser_impl::parse as manual_parse;
// use crate::parser_nom::parser_impl::parse as nom_parse;
use crate::parser::parser_impl::parse_tokens as chumsky_parse;

#[derive(Debug)]
enum LexChoice {
    Manual,
    Logos,
}

#[derive(Debug)]
enum ParseChoice {
    Manual,
    Nom,
    Chumsky,
}

fn main() {
    let code = r#"
    if (hero + 10) { 
        // Se hero + 10 for verdadeiro, mova para a esquerda e verifique o inimigo
        move_left 
        if (enemy - 2) { attack } else { defend }
    } else { 
        jump 
    }
    
    while (enemy - 5) { 
        // Enquanto houver inimigo (com ajuste), ataque e mova para a direita
        attack 
        move_right 
    }
    
    for (hero; enemy; 3) { 
        // Loop: inicializa com 'hero', condição 'enemy' e atualização '3'
        move_up 
        if (treasure) { 
            move_right 
            attack 
        } else { 
            jump 
        }
    }
    "#;

    let code = r#"
        if (hero + 10 { 
            move_up 
        } else { 
            jump 
        }
        "#;

    

    let lex_choice = choose_lex();
    let parse_choice = choose_parser();

    // 1) Análise Léxica
    let tokens = match lex_choice {
        LexChoice::Manual => {
            println!("\n[INFO] Usando lexer manual...");
            manual_lex(code)
        }
        LexChoice::Logos => {
            println!("\n[INFO] Usando lexer Logos...");
            logos_lex(code)
        }
    };

    println!("\nTokens gerados:\n{:?}", tokens);

    // 2) Análise Sintática
    match parse_choice {
        ParseChoice::Manual => {
            println!("\n[INFO] Usando parser manual...");
            match manual_parse(tokens) {
                Ok(ast) => println!("AST gerada com sucesso!\n{:#?}", ast),
                Err(e) => eprintln!("Erro no parser manual: {:?}", e),
            }
        }
        ParseChoice::Nom => {
            println!("\n[INFO] Usando parser com Nom...");
            /* 
            match nom_parse(tokens) {
                Ok((rest, ast)) => {
                    println!("AST gerada com sucesso!\n{:#?}", ast);
                    println!("Restante não parseado: {:?}", rest);
                }
                Err(e) => eprintln!("Erro no parser Nom: {:?}", e),
            }
            */
        }
        ParseChoice::Chumsky => {
            println!("\n[INFO] Usando parser com Chumsky...");
            match chumsky_parse(tokens) {
                Ok(ast) => println!("AST gerada com sucesso!\n{:#?}", ast),
                Err(errors) => {
                    eprintln!("Erros de parsing:");
                    for err in errors {
                        eprintln!("{:?}", err);
                    }
                }
            }
        }
    }
}

fn choose_lex() -> LexChoice {
    println!("Selecione o Analisador Léxico:");
    println!("1) Lexer Manual");
    println!("2) Lexer Logos");
    print!("Escolha: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => LexChoice::Manual,
        "2" => LexChoice::Logos,
        _ => {
            println!("Opção inválida. Usando Lexer Manual como padrão.");
            LexChoice::Manual
        }
    }
}

fn choose_parser() -> ParseChoice {
    println!("\nSelecione o Analisador Sintático:");
    println!("1) Parser Manual");
    println!("2) Parser Nom");
    println!("3) Parser Chumsky");
    print!("Escolha: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => ParseChoice::Manual,
        "2" => ParseChoice::Nom,
        "3" => ParseChoice::Chumsky,
        _ => {
            println!("Opção inválida. Usando Parser Manual como padrão.");
            ParseChoice::Manual
        }
    }
}
