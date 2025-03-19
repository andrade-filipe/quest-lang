#[derive(Debug, PartialEq)]
pub enum Statement {
    Command(Command),
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Box<Statement>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        init: Expression,
        condition: Expression,
        update: Expression,
        body: Box<Statement>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Defend,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(String),
    Number(i64),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Plus,
    Minus,
}
