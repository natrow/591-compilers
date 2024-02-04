use std::collections::{HashMap, HashSet};

use crate::nfa::Nfa;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NfaError(crate::nfa::Error<usize, char>),
}

impl From<crate::nfa::Error<usize, char>> for Error {
    fn from(value: crate::nfa::Error<usize, char>) -> Self {
        Error::NfaError(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn parse_regex<S>(regex: S) -> Result<Nfa<usize, char>>
where
    S: IntoIterator<Item = char>,
{
    let states = Vec::new();
    let alphabet = Vec::new();

    let edges = HashMap::new();

    let initial = 0;

    let accepting = [].into();

    Ok(Nfa::new(
        HashSet::from_iter(states.into_iter()),
        HashSet::from_iter(alphabet.into_iter()),
        edges,
        initial,
        accepting,
    )?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Regex {
    Char(char),
    Episillon,
    Or(Box<Regex>, Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
    ZeroOrMore(Box<Regex>),
    Class(HashSet<char>),
}
