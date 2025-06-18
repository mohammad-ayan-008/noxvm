#[derive(Debug, PartialEq, Clone, Default)]
pub enum Kind {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    IdentifierLiteral,
    StringLiteral,
    NumberLiteral,

    And,
    Or,
    Class,
    Fun,
    Var,
    If,
    Else,
    While,
    For,
    True,
    False,
    Nil,
    Print,
    Return,
    Super,
    This,

    #[default]
    Eof,
    Error,
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub kind: Kind,
    pub line: usize,
    pub index_in_source: usize,
    pub string: String,
}
