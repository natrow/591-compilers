use crate::scanner::*;
use std::{collections::HashSet, iter::Iterator as IteratorTrait};

pub enum Tree {
    Concat(Box<Tree>, Box<Tree>),
    Or(Box<Tree>, Box<Tree>),
    Any,
    Char(char),
    Not(Box<Tree>),
    Repeat(Box<Tree>),
    Epsillon,
    Class(HashSet<char>),
}

pub struct Iterator<T>
where
    T: IteratorTrait<Item = Token>,
{
    tokens: T,
}

impl<T> Iterator<T>
where
    T: IteratorTrait<Item = Token>,
{
    pub fn new<I>(tokens: I) -> Self
    where
        I: IntoIterator<IntoIter = T>,
    {
        Self {
            tokens: tokens.into_iter(),
        }
    }
}

impl<T> IteratorTrait for Iterator<T>
where
    T: IteratorTrait<Item = Token>,
{
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_token) = self.tokens.next() {
            match self.tokens.next()? {
                Token::Char(_) => todo!(),
                Token::Any => todo!(),
                Token::BinOp(_) => todo!(),
                Token::PrefixOp(_) => todo!(),
                Token::PostfixOp(_) => todo!(),
                Token::LParen => todo!(),
                Token::RParen => todo!(),
                Token::LBracket => todo!(),
                Token::RBracket => todo!(),
                Token::Escape => todo!(),
                Token::Through => todo!(),
            }
        } else {
            None
        }
    }
}
