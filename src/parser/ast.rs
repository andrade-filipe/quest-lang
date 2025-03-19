#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Defend,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Number(i64),
    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
}