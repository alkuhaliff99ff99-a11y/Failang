#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,

    LessThan,
    GreaterThan,

    LessOrEqual,
    GreaterOrEqual,
}


#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
}
