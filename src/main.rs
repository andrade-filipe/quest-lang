mod lexer;

use lexer::token::Token;
use logos::Logos;

fn main() {
    let input = "if (hero + 10) { move_up } // comentário exemplo";
    let lex = Token::lexer(input);

    // Itera sobre os tokens e os imprime
    for token in lex {
        println!("{:?}", token);
    }
}
