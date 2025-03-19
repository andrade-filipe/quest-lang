#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Comandos de Movimento
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    // Comandos de Ação
    Jump,
    Attack,
    Defend,

    // Estruturas de Controle
    If,
    Else,
    While,
    For,

    // Operadores Aritméticos
    Plus,
    Minus,
    Asterisk,
    Slash,

    // Operadores Lógicos
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Delimitadores
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Literais e Identificadores
    Identifier(String),
    Number(i64),

    // Comentário
    Comment(String),
    
    // Fim da Entrada
    EOF,
}
