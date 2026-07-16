#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Array,
    Function,
    Nil,
    Unknown,
}
