use logos::Logos;

#[derive(Logos, Default, Debug, PartialEq)]
pub enum Token {
    // Comandos de Movimento
    #[token("move_up")]
    MoveUp,
    #[token("move_down")]
    MoveDown,
    #[token("move_left")]
    MoveLeft,
    #[token("move_right")]
    MoveRight,

    // Comandos de Ação
    #[token("jump")]
    Jump,
    #[token("attack")]
    Attack,
    #[token("defend")]
    Defend,

    // Estruturas de Controle
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,

    // Operadores Aritméticos
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,

    // Operadores Lógicos
    #[token("&&")]
    LogicalAnd,
    #[token("||")]
    LogicalOr,
    #[token("!")]
    LogicalNot,

    // Delimitadores
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    // Literais e Identificadores
    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Number(i64),
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // Comentário
    #[regex("//[^\n]*", |lex| lex.slice().to_string())]
    Comment(String),

    // Ignora espaços em branco e outros separadores.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[default]
    Error,
}
