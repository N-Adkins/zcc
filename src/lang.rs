use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

#[derive(Debug)]
pub enum Constant {
    Floating(f64),
    Integer(i64),
    Enumeration(i64),
    Character(char),
}

#[derive(Debug)]
pub enum Operator {
    LBracket,
    RBracket,
    LParen,
    RParen,
    Period,
    Arrow,
    DoublePlus,
    DoubleMinus,
    Ampersand,
    Asterisk,
    Plus,
    Minus,
    Tilde,
    Exclaimation,
    Sizeof,
    ForwardSlash,
    Percent,
    BitwiseLeft,
    BitwiseRight,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    Equality,
    NotEquality,
    BitwiseXor,
    BitwiseOr,
    BooleanAnd,
    BooleanOr,
    QuestionMark,
    Colon,
    Equals,
    MultiplyEquals,
    DivideEquals,
    ModulusEquals,
    PlusEquals,
    MinusEquals,
    BitwiseLeftEquals,
    BitwiseRightEquals,
    BitwiseAndEquals,
    BitwiseXorEquals,
    BitwiseOrEquals,
    Comma,
    Pound,
    DoublePound,
}

#[derive(Debug)]
pub enum Punctuator {
    LBracket,
    RBracket,
    LParen,
    RParen,
    LCurly,
    RCurly,
    Asterisk,
    Comma,
    Colon,
    Equals,
    Semicolon,
    TripleDot,
    Pound,
}

lazy_static! {
    pub static ref KEYWORD_MAP: HashMap<&'static str, Keyword> = HashMap::from([
        ("auto", Keyword::Auto),
        ("break", Keyword::Break),
        ("case", Keyword::Case),
        ("char", Keyword::Char),
        ("const", Keyword::Const),
        ("continue", Keyword::Continue),
        ("default", Keyword::Default),
        ("do", Keyword::Do),
        ("double", Keyword::Double),
        ("else", Keyword::Else),
        ("enum", Keyword::Enum),
        ("extern", Keyword::Extern),
        ("float", Keyword::Float),
        ("for", Keyword::For),
        ("goto", Keyword::Goto),
        ("if", Keyword::If),
        ("int", Keyword::Int),
        ("long", Keyword::Long),
        ("register", Keyword::Register),
        ("return", Keyword::Return),
        ("short", Keyword::Short),
        ("signed", Keyword::Signed),
        ("sizeof", Keyword::Sizeof),
        ("static", Keyword::Static),
        ("struct", Keyword::Struct),
        ("switch", Keyword::Switch),
        ("typedef", Keyword::Typedef),
        ("union", Keyword::Union),
        ("unsigned", Keyword::Unsigned),
        ("void", Keyword::Void),
        ("volatile", Keyword::Volatile),
        ("while", Keyword::While),
    ]);
    pub static ref OPERATOR_MAP: HashMap<&'static str, Operator> = HashMap::from([
        ("[", Operator::LBracket),
        ("]", Operator::RBracket),
        ("(", Operator::LParen),
        (")", Operator::RParen),
        (".", Operator::Period),
        ("->", Operator::Arrow),
        ("++", Operator::DoublePlus),
        ("--", Operator::DoubleMinus),
        ("&", Operator::Ampersand),
        ("*", Operator::Asterisk),
        ("+", Operator::Plus),
        ("-", Operator::Minus),
        ("~", Operator::Tilde),
        ("!", Operator::Exclaimation),
        ("sizeof", Operator::Sizeof),
        ("/", Operator::ForwardSlash),
        ("%", Operator::Percent),
        ("<<", Operator::BitwiseLeft),
        (">>", Operator::BitwiseRight),
        ("<", Operator::LessThan),
        (">", Operator::GreaterThan),
        ("<=", Operator::LessThanEquals),
        (">=", Operator::GreaterThanEquals),
        ("==", Operator::Equality),
        ("!=", Operator::NotEquality),
        ("^", Operator::BitwiseXor),
        ("|", Operator::BitwiseOr),
        ("&&", Operator::BooleanAnd),
        ("||", Operator::BooleanOr),
        ("?", Operator::QuestionMark),
        (";", Operator::Colon),
        ("=", Operator::Equals),
        ("*=", Operator::MultiplyEquals),
        ("/=", Operator::DivideEquals),
        ("%=", Operator::ModulusEquals),
        ("+=", Operator::PlusEquals),
        ("-=", Operator::MinusEquals),
        ("<<=", Operator::BitwiseLeftEquals),
        (">>=", Operator::BitwiseRightEquals),
        ("&=", Operator::BitwiseAndEquals),
        ("^=", Operator::BitwiseXorEquals),
        ("|=", Operator::BitwiseOrEquals),
        (",", Operator::Comma),
        ("#", Operator::Pound),
        ("##", Operator::DoublePound),
    ]);
}
