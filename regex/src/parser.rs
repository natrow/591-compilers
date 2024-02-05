// use std::collections::{HashMap, HashSet};

// use log::trace;

// use crate::nfa::Nfa;

// #[derive(Debug, PartialEq, Eq)]
// pub enum Error {
//     NfaError(crate::nfa::Error<usize, char>),
//     EmptyRegex,
//     UnterminatedEscape(usize),
//     UnterminatedNot(usize),
//     UnterminatedParen(usize),
//     EmptyGroup(usize),
// }

// impl From<crate::nfa::Error<usize, char>> for Error {
//     fn from(value: crate::nfa::Error<usize, char>) -> Self {
//         Error::NfaError(value)
//     }
// }

// pub type Result<T> = core::result::Result<T, Error>;

// pub fn parse_regex<T>(s: T) -> Result<Nfa<usize, char>>
// where
//     T: IntoIterator<Item = char>,
// {
//     let s = s.into_iter().enumerate();
//     let _regex = match munch_regex(s)? {
//         Termination::Complete(r) => r,
//         Termination::Incomplete(_r, _c, _s) => todo!(),
//         Termination::Empty => return Err(Error::EmptyRegex),
//     };

//     let states = Vec::new();
//     let alphabet = Vec::new();

//     let edges = HashMap::new();

//     let initial = 0;

//     let accepting = [].into();

//     Ok(Nfa::new(
//         HashSet::from_iter(states.into_iter()),
//         HashSet::from_iter(alphabet.into_iter()),
//         edges,
//         initial,
//         accepting,
//     )?)
// }

// // Ways in which the muncher can return; this is necessary for the recursive/iterator implementation
// #[derive(Debug, PartialEq, Eq)]
// pub enum Termination<T>
// where
//     T: Iterator<Item = (usize, char)>,
// {
//     /// Parser consumed entire Regex, no errors
//     Complete(Regex),
//     /// Parser couldn't complete the Regex, possible error
//     Incomplete(Option<Regex>, char, T),
//     /// Iterator was already empty
//     Empty,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Regex {
//     Char(char),
//     Epsillon,
//     Any,
//     Or(Box<Regex>, Box<Regex>),
//     Concat(Box<Regex>, Box<Regex>),
//     Repeat(Box<Regex>),
//     Class(HashSet<char>),
//     Not(Box<Regex>),
// }

// Regular expressions metalanguage:
// -a (char)
// | (or)
// -. (any)
// ? (0-1)
// -* (0+)
// + (1+)
// -() (group)
// [] (character class)
// -\ (escape metalanguage)

// pub fn munch_regex<T>(mut s: T) -> Result<Termination<T>>
// where
//     T: Iterator<Item = (usize, char)>,
// {
//     let r = match s.next() {
//         Some((i, c)) => match c {
//             '\\' => match s.next() {
//                 // todo: special characters
//                 Some((_, c)) => concat_regex(Regex::Char(c), s)?,
//                 None => return Err(Error::UnterminatedEscape(i)),
//             },
//             '(' => match munch_regex(s)? {
//                 Termination::Complete(_) | Termination::Empty => {
//                     return Err(Error::UnterminatedParen(i))
//                 }
//                 Termination::Incomplete(inner, c, s) => match c {
//                     ')' => {
//                         if let Some(inner) = inner {
//                             concat_regex(inner, s)?
//                         } else {
//                             return Err(Error::EmptyGroup(i));
//                         }
//                     }
//                     _ => todo!(),
//                 },
//             },
//             '.' => concat_regex(Regex::Any, s)?,
//             ')' | '*' => Termination::Incomplete(None, c, s),
//             c => concat_regex(Regex::Char(c), s)?,
//         },
//         None => Termination::Empty,
//     };

//     Ok(r)
// }

// fn concat_regex<T>(first: Regex, mut s: T) -> Result<Termination<T>>
// where
//     T: Iterator<Item = (usize, char)>,
// {
//     trace!("concatenating {:?}", first);

//     let r = match munch_regex(s)? {
//         Termination::Complete(r) => {
//             Termination::Complete(Regex::Concat(Box::new(first), Box::new(r)))
//         }
//         Termination::Incomplete(r, c, s) => {
//             if let Some(r) = r {
//                 trace!("incomplete+concat {:?}", r);
//                 Termination::Incomplete(Some(Regex::Concat(Box::new(first), Box::new(r))), c, s)
//             } else {
//                 trace!("incomplete/finish");
//                 match c {
//                     // precedence rules: paren -> operators -> concat
//                     '*' => concat_regex(Regex::Repeat(Box::new(first)), s)?,
//                     ')' => Termination::Incomplete(Some(first), c, s),
//                     _ => panic!("unknown special char"),
//                 }
//             }
//         }
//         Termination::Empty => Termination::Complete(first),
//     };

//     Ok(r)
// }
