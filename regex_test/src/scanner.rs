pub enum BinOp {
    Or,
}

pub enum PrefixOp {
    Not,
}

pub enum PostfixOp {
    Maybe,
    Repeating,
    AtLeastOne,
}

pub enum Token {
    Char(char),
    Any,
    BinOp(BinOp),
    PrefixOp(PrefixOp),
    PostfixOp(PostfixOp),
    LParen,
    RParen,
    LBracket,
    RBracket,
    Escape,
    Through,
}

pub fn scan_token(c: char) -> Token {
    match c {
        '.' => Token::Any,
        '^' => Token::PrefixOp(PrefixOp::Not),
        '|' => Token::BinOp(BinOp::Or),
        '?' => Token::PostfixOp(PostfixOp::Maybe),
        '*' => Token::PostfixOp(PostfixOp::Repeating),
        '+' => Token::PostfixOp(PostfixOp::AtLeastOne),
        '(' => Token::LParen,
        ')' => Token::RParen,
        '[' => Token::LBracket,
        ']' => Token::RBracket,
        '\\' => Token::Escape,
        '-' => Token::Through,
        x => Token::Char(x),
    }
}
