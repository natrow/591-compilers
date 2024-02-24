//! Determines whether a given context-free grammar is LL(1).
//!
//! Rules:
//! 1. For each non-terminal A, the first of each production of A must be disjoint (implies a lack of left-recursion).
//!
//! A -> B1 | B2 | ... | Bn
//! FIRST(Bj) union FIRST(Bk) = {} for all j != k
//!
//! a is in the set FIRST(A) if Bn -> aC for some n
//!
//! 2. For each non-terminal A that can generate an empty string, the first and the follow of A must be disjoint.
//!
//! FIRST(A) union FOLLOW(A) = {}
//!
//! Given Pk -> Bk A Ck then FOLLOW(A) = FIRST(C1) union FIRST(C2) union ... union FIRST(Cn)
//!
//! and if there exists some Ck -> {} then FOLLOW(A) also includes FOLLOW(Pk)

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use log::trace;

use crate::cfg::{ContextFreeGrammar, Productions, Symbol};

/// Max recursion depth
const RECURSION_LIMIT: usize = 50;

/// Errors that prevent a context-free grammar from being LL(1)
#[derive(Debug)]
pub enum Error<N: Eq + Hash + Clone> {
    /// A nonterminal failed rule 1
    Rule1(N),
    /// A nonterminal failed rule 2
    Rule2(N),
}

/// An LL(1) grammar. This is a context-free grammar which has been verified to follow the rules.
pub struct LL1<T: Eq + Hash + Clone, N: Eq + Hash + Clone> {
    /// Definition of the grammar
    _cfg: ContextFreeGrammar<T, N>,
}

impl<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> LL1<T, N> {
    /// Determines whether a nonterminal violates the first rule of LL(1) grammars.
    fn llk_rule_1(
        memoize: &mut Memoize<T, N>,
        productions: &Productions<T, N>,
        n: &N,
    ) -> Result<(), Error<N>> {
        let mut terminals = HashSet::new();

        for rhs in productions.get(n).unwrap() {
            let first = memoize.first_of_rhs(productions, rhs, RECURSION_LIMIT);
            if !terminals.is_disjoint(&first) {
                return Err(Error::Rule1(n.clone()));
            }
            terminals.extend(first)
        }

        Ok(())
    }

    /// Determines whether a nonterminal violates the second rule of LL(1) grammars.
    fn llk_rule_2(
        memoize: &mut Memoize<T, N>,
        productions: &Productions<T, N>,
        n: &N,
    ) -> Result<(), Error<N>> {
        if memoize.nonterminal_generates_empty(productions, n, RECURSION_LIMIT) {
            let first = memoize.first_of_nonterminal(productions, n, RECURSION_LIMIT);
            let follow = memoize.follow_of_nonterminal(productions, n, RECURSION_LIMIT);

            if !first.is_disjoint(&follow) {
                return Err(Error::Rule2(n.clone()));
            }
        }

        Ok(())
    }

    /// Determines whether the context-free grammar is LL(1). Returns Ok() if true, or
    /// an error explaining why not.
    pub fn new(cfg: ContextFreeGrammar<T, N>) -> Result<Self, Error<N>> {
        // initialize Memoize struct
        let mut memoize = Memoize::default();

        // get productions
        let productions = cfg.get_productions();

        // calculate first sets
        for n in cfg.get_nonterminals() {
            let set = memoize.first_of_nonterminal(productions, n, RECURSION_LIMIT);
            trace!("FIRST({:?}) = {:?}", n, set);
        }
        // calculate follow sets
        for n in cfg.get_nonterminals() {
            let set = memoize.follow_of_nonterminal(productions, n, RECURSION_LIMIT);
            trace!("FOLLOW({:?}) = {:?}", n, set);
        }

        // apply rule 1
        cfg.get_nonterminals()
            .iter()
            .try_for_each(|n| Self::llk_rule_1(&mut memoize, productions, n))?;

        // apply rule 2
        cfg.get_nonterminals()
            .iter()
            .try_for_each(|n| Self::llk_rule_2(&mut memoize, productions, n))?;

        Ok(Self { _cfg: cfg })
    }
}

/// Memoized look-up tables of calculated values.
#[derive(Debug)]
struct Memoize<T: Eq + Hash + Clone, N: Eq + Hash + Clone> {
    /// Memoized result of the first() function on a nonterminal
    first: HashMap<N, HashSet<T>>,
    /// Memoized result of the follow() function on a nonterminal
    follow: HashMap<N, HashSet<T>>,
    /// Whether or not a nonterminal can generate the empty string
    generates_empty: HashMap<N, bool>,
}

impl<T: Eq + Hash + Clone, N: Eq + Hash + Clone> Default for Memoize<T, N> {
    fn default() -> Self {
        Self {
            first: Default::default(),
            follow: Default::default(),
            generates_empty: Default::default(),
        }
    }
}

impl<T: Eq + Hash + Clone, N: Eq + Hash + Clone> Memoize<T, N> {
    /// Determines whether a symbol can generate the empty string. Terminals never do this.
    fn symbol_generates_empty(
        &mut self,
        productions: &Productions<T, N>,
        s: &Symbol<T, N>,
        recursion_limit: usize,
    ) -> bool {
        match s {
            Symbol::Nonterminal(n) => {
                self.nonterminal_generates_empty(productions, n, recursion_limit)
            }
            Symbol::Terminal(_) => false,
        }
    }

    /// Determines whether a nonterminal can generate the empty string.
    fn nonterminal_generates_empty(
        &mut self,
        productions: &Productions<T, N>,
        n: &N,
        recursion_limit: usize,
    ) -> bool {
        // 1. Already in the set
        if let Some(v) = self.generates_empty.get(n) {
            return *v;
        }

        // 2. A production contains the empty string
        if productions.get(n).unwrap().iter().any(|v| v.is_empty()) {
            self.generates_empty.insert(n.clone(), true);
            return true;
        }

        assert!(
            recursion_limit > 0,
            "recursion limit reached in Memoize::nonterminal_generates_empty"
        );

        // 3. A production can generate the empty string
        for rhs in productions.get(n).unwrap().iter() {
            if rhs
                .iter()
                .all(|s| self.symbol_generates_empty(productions, s, recursion_limit - 1))
            {
                self.generates_empty.insert(n.clone(), true);
                return true;
            }
        }

        self.generates_empty.insert(n.clone(), false);
        false
    }

    /// Determines the result of first() for a symbol. Terminals always return themselves.
    fn first_of_symbol(
        &mut self,
        productions: &Productions<T, N>,
        s: &Symbol<T, N>,
        recursion_limit: usize,
    ) -> HashSet<T> {
        match s {
            Symbol::Nonterminal(n) => self.first_of_nonterminal(productions, n, recursion_limit),
            Symbol::Terminal(t) => HashSet::from([t.clone()]),
        }
    }

    /// Determines the result of first() for a right-hand-side of a production rule.
    fn first_of_rhs(
        &mut self,
        productions: &Productions<T, N>,
        rhs: &[Symbol<T, N>],
        recursion_limit: usize,
    ) -> HashSet<T> {
        let mut set = HashSet::new();

        for s in rhs {
            set.extend(self.first_of_symbol(productions, s, recursion_limit));
            // repeat until a symbol that cannot be an empty string has been reached
            if !self.symbol_generates_empty(productions, s, RECURSION_LIMIT) {
                break;
            }
        }

        set
    }

    /// Determines the result of first() for a nonterminal, using all of its production rules.
    fn first_of_nonterminal(
        &mut self,
        productions: &Productions<T, N>,
        n: &N,
        recursion_limit: usize,
    ) -> HashSet<T> {
        // if the first set is already calculated, return it
        if let Some(v) = self.first.get(n) {
            return v.clone();
        }

        // otherwise calculate the set
        let mut set = HashSet::new();

        // expand each production
        for rhs in productions.get(n).unwrap() {
            assert!(
                recursion_limit > 0,
                "recursion limit reached in Memoize::first_of_nonterminal()"
            );

            set.extend(self.first_of_rhs(productions, rhs, recursion_limit - 1));
        }

        self.first.insert(n.clone(), set.clone());
        set
    }

    /// Determines the result of follow() for a nonterminal, using all production rules of the grammar.
    fn follow_of_nonterminal(
        &mut self,
        productions: &Productions<T, N>,
        n: &N,
        recursion_limit: usize,
    ) -> HashSet<T> {
        // if the follow set is already calculated, return it
        if let Some(v) = self.follow.get(n) {
            return v.clone();
        }

        // otherwise calculate the set
        let mut set = HashSet::new();

        for p in productions
            .iter()
            .flat_map(|p| p.1.iter().map(move |s| (p.0, s)))
        {
            for (i, s) in p.1.iter().enumerate() {
                if s == n {
                    // if the rest of rhs is non-empty, add the first terminal in the remainder of rhs
                    if i + 1 < p.1.len() {
                        set.extend(self.first_of_rhs(productions, &p.1[i + 1..], RECURSION_LIMIT));

                        // if the rest of the rhs can generate the empty string, append the follow of the lhs of the production
                        if p.0 != n
                            && p.1[i + 1..].iter().all(|s| {
                                self.symbol_generates_empty(productions, s, RECURSION_LIMIT)
                            })
                        {
                            assert!(
                                recursion_limit > 0,
                                "recursion limit reached in Memoize::follow_of_nonterminal()"
                            );

                            set.extend(self.follow_of_nonterminal(
                                productions,
                                p.0,
                                recursion_limit - 1,
                            ))
                        }
                    } else if p.0 != n {
                        assert!(
                            recursion_limit > 0,
                            "recursion limit reached in Memoize::follow_of_nonterminal()"
                        );

                        // otherwise append the follow of the lhs of the production
                        set.extend(self.follow_of_nonterminal(
                            productions,
                            p.0,
                            recursion_limit - 1,
                        ))
                    }
                }
            }
        }

        self.follow.insert(n.clone(), set.clone());
        set
    }
}
