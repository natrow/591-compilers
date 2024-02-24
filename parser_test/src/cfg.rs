//! Allows the specification of a context-free grammar.
//!
//! Also validates whether the grammar is well-defined.

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Some non-terminal
pub type Terminals<T> = HashSet<T>;
/// Some terminal
pub type Nonterminals<N> = HashSet<N>;
/// Some production in the form A -> alpha
pub type Production<T, N> = (N, Vec<Symbol<T, N>>);
/// The set of productions in a context-free grammar
pub type Productions<T, N> = HashMap<N, HashSet<Vec<Symbol<T, N>>>>;

/// A single symbol in a language, which may or may not be terminal
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Symbol<T: Eq + Clone, N: Eq + Clone> {
    /// A terminal symbol, i.e. one that maps directly to a token
    Terminal(T),
    /// A non-terminal symbol, e.g. a null string, sequence of terminals and/or nonterminals, etc.
    Nonterminal(N),
}

impl<T: Eq + Clone, N: Eq + Clone> PartialEq<N> for Symbol<T, N> {
    fn eq(&self, other: &N) -> bool {
        match self {
            Self::Nonterminal(n) => n == other,
            Self::Terminal(_) => false,
        }
    }
}

/// Errors that can occur during evaluation of the grammar
#[derive(Debug)]
pub enum Error<T: Eq + Clone, N: Eq + Clone> {
    /// A production rule contained a terminal symbol that was not in the set of terminals
    UnknownTerminalInProduction(T, Production<T, N>),
    /// A production rule contained a nonterminal symbol that was not in the set of nonterminals
    UnknownNonterminalInProduction(N, Production<T, N>),
    /// A nonterminal symbol was missing production rules
    MissingProductionsForNonterminal(N),
}

/// A struct representing the semantics of a context-free grammar.
#[derive(Debug, Clone)]
pub struct ContextFreeGrammar<T: Eq + Hash + Clone, N: Eq + Hash + Clone> {
    /// The set of terminal symbols
    terminals: Terminals<T>,
    /// The set of nonterminal symbols
    nonterminals: Nonterminals<N>,
    /// The production rules for nonterminal symbols
    productions: Productions<T, N>,
}

impl<T: Eq + Hash + Clone, N: Eq + Hash + Clone> ContextFreeGrammar<T, N> {
    /// Validates whether a nonterminal is in the set of nonterminals
    fn validate_nonterminal(
        n: &N,
        nonterminals: &Nonterminals<N>,
        p: &Production<T, N>,
    ) -> Result<(), Error<T, N>> {
        if nonterminals.contains(n) {
            Ok(())
        } else {
            Err(Error::UnknownNonterminalInProduction(n.clone(), p.clone()))
        }
    }

    /// Validates whether a terminal is in the set of terminals
    fn validate_terminal(
        t: &T,
        terminals: &Terminals<T>,
        p: &Production<T, N>,
    ) -> Result<(), Error<T, N>> {
        if terminals.contains(t) {
            Ok(())
        } else {
            Err(Error::UnknownTerminalInProduction(t.clone(), p.clone()))
        }
    }

    /// Validates whether a symbol is in the appropriate set of symbols
    fn validate_symbol(
        s: &Symbol<T, N>,
        terminals: &Terminals<T>,
        nonterminals: &Nonterminals<N>,
        p: &Production<T, N>,
    ) -> Result<(), Error<T, N>> {
        match s {
            Symbol::Terminal(t) => Self::validate_terminal(t, terminals, p),
            Symbol::Nonterminal(n) => Self::validate_nonterminal(n, nonterminals, p),
        }
    }

    /// Validates whether all symbols in a production rule are valid in their appropriate set of symbols
    fn validate_production(
        terminals: &Terminals<T>,
        nonterminals: &Nonterminals<N>,
        p: &Production<T, N>,
    ) -> Result<(), Error<T, N>> {
        Self::validate_nonterminal(&p.0, nonterminals, p)?;

        for symbol in p.1.iter() {
            Self::validate_symbol(symbol, terminals, nonterminals, p)?;
        }

        Ok(())
    }

    /// Constructs the context-free grammar, while evaluating its validity.
    pub fn new(
        terminals: Terminals<T>,
        nonterminals: Nonterminals<N>,
        productions: Productions<T, N>,
    ) -> Result<Self, Error<T, N>> {
        // rules to check:

        // 1. every symbol in every production must exist in the set of terminals and nonterminals
        // 2. every nonterminal must have at least one production
        // 3. every production must be unique (guaranteed by type)

        // validate rule 1
        productions
            .iter()
            .flat_map(|p| p.1.iter().map(|rhs| (p.0.clone(), rhs.clone())))
            .try_for_each(|p| Self::validate_production(&terminals, &nonterminals, &p))?;

        // validate rule 2
        for n in nonterminals.iter() {
            if !productions.contains_key(n) {
                return Err(Error::MissingProductionsForNonterminal(n.clone()));
            }
        }

        // initialize grammar
        let grammar = Self {
            terminals,
            nonterminals,
            productions,
        };

        Ok(grammar)
    }

    /// Get the set of nonterminals
    pub fn get_nonterminals(&self) -> &Nonterminals<N> {
        &self.nonterminals
    }

    /// Get the set of terminals
    pub fn get_terminals(&self) -> &Terminals<T> {
        &self.terminals
    }

    /// Get the set of production rules
    pub fn get_productions(&self) -> &Productions<T, N> {
        &self.productions
    }
}
