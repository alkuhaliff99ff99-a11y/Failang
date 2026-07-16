use super::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        initializer: Expression,
    },
    Expression {
        expression: Expression,
    },
    Print {
        expression: Expression,
    },
    Block {
        statements: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Statement>,
    },
    Return {
        value: Option<Expression>,
    },
}
