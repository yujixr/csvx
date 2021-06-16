mod parse;
mod primitive_parse;

pub use parse::parse;
pub use primitive_parse::primitive_parse;

#[derive(Clone, PartialEq)]
pub enum PrimitiveToken {
    Parsed(Token),
    Raw(String),
}

#[derive(Clone, PartialEq)]
pub enum Token {
    // Value
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    // Reference
    Ref(usize, usize),
    Range(usize, usize, usize, usize),
    // Variable
    Var(String),
    // Operator
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    Xor,
    RightShift,
    LeftShift,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    // Parenthesis
    ParenthesisBegin,
    ParenthesisEnd,
    Comma,
    // Functions
    FnRef,
    FnSum,
    FnAvg,
    FnIf,
    FnRound,
    FnFloor,
    FnCeil,
    FnLog,
    FnLn,
    FnLog2,
    FnLog10,
    FnSqrt,
    FnPow,
    FnSin,
    FnCos,
    FnTan,
    FnAsin,
    FnAcos,
    FnAtan,
    FnSinh,
    FnCosh,
    FnTanh,
    FnAsinh,
    FnAcosh,
    FnAtanh,
}
