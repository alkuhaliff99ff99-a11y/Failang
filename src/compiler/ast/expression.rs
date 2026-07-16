use super::operator::{BinaryOperator, UnaryOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),

    Identifier(String),

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },

    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },

    ArrayLiteral(Vec<Expression>),

    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },

    IndexAssign {
        array: Box<Expression>,
        index: Box<Expression>,
        value: Box<Expression>,
    },
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
