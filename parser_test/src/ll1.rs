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
    ops::Deref,
};

use crate::{
    cfg::{ContextFreeGrammar, Productions, Symbol},
    compute::{compute_first, compute_follow, compute_predict_sets},
};

/// Errors that prevent a context-free grammar from being LL(1)
#[derive(Debug)]
pub enum Error<N: Eq + Hash + Clone + Debug> {
    /// A nonterminal failed rule 1
    Rule1(N),
    /// A nonterminal failed rule 2
    Rule2(N),
}

/// An LL(1) grammar. This is a context-free grammar which has been verified to follow the rules.
pub struct LL1<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> {
    /// Definition of the grammar
    cfg: ContextFreeGrammar<T, N>,
    /// Memoization table of calculations
    memoize: CachedMaps<T, N>,
}

impl<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> LL1<T, N> {
    /// Determines whether a nonterminal violates the first rule of LL(1) grammars.
    fn llk_rule_1(
        memoize: &mut CachedMaps<T, N>,
        productions: &Productions<T, N>,
        n: &N,
    ) -> Result<(), Error<N>> {
        let mut terminals = HashSet::new();

        for rhs in productions.get(n).unwrap() {
            let first = memoize.first_of_rhs(rhs);
            if !terminals.is_disjoint(&first) {
                return Err(Error::Rule1(n.clone()));
            }
            terminals.extend(first)
        }

        Ok(())
    }

    /// Determines whether a nonterminal violates the second rule of LL(1) grammars.
    fn llk_rule_2(memoize: &mut CachedMaps<T, N>, n: &N) -> Result<(), Error<N>> {
        if memoize.nonterminal_generates_empty(n) {
            let first = memoize.first_of_nonterminal(n);
            let follow = memoize.follow_of_nonterminal(n);

            if !first.is_disjoint(follow) {
                println!(
                    "{:?} failed rule 2: (first = {:?}, follow = {:?})",
                    n, first, follow
                );
                return Err(Error::Rule2(n.clone()));
            }
        }

        Ok(())
    }

    /// Determines whether the context-free grammar is LL(1). Returns Ok() if true, or
    /// an error explaining why not.
    pub fn new(cfg: ContextFreeGrammar<T, N>) -> Result<Self, Vec<Error<N>>> {
        // initialize Memoize struct
        let mut memoize = CachedMaps::new(&cfg);

        // get productions
        let productions = cfg.get_productions();

        // apply rule 1
        let mut errs: Vec<Error<N>> = cfg
            .get_nonterminals()
            .iter()
            .filter_map(|n| Self::llk_rule_1(&mut memoize, productions, n).err())
            .collect();

        // apply rule 2
        errs.extend(
            cfg.get_nonterminals()
                .iter()
                .filter_map(|n| Self::llk_rule_2(&mut memoize, n).err()),
        );

        if errs.is_empty() {
            Ok(Self { cfg, memoize })
        } else {
            Err(errs)
        }
    }

    /// Get the first sets of each non-terminal
    pub fn get_first_sets(&self) -> &HashMap<N, HashSet<T>> {
        &self.memoize.first_sets
    }

    /// Get the first sets of each non-terminal
    pub fn get_follow_sets(&self) -> &HashMap<N, HashSet<T>> {
        &self.memoize.follow_sets
    }

    /// Get the predict sets of each non-terminal
    pub fn get_predict_sets(&self) -> &HashMap<N, HashSet<T>> {
        &self.memoize.predict_sets
    }
}

impl<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> Deref for LL1<T, N> {
    type Target = ContextFreeGrammar<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}

/// Cached maps of each nonterminal to the corresponding set
#[derive(Debug)]
struct CachedMaps<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> {
    /// Cached result of the first() function on a nonterminal
    first_sets: HashMap<N, HashSet<T>>,
    /// Cached result of the follow() function on a nonterminal
    follow_sets: HashMap<N, HashSet<T>>,
    /// Whether or not a nonterminal can generate the empty string
    generates_empty: HashMap<N, bool>,
    /// Predict sets
    predict_sets: HashMap<N, HashSet<T>>,
}

impl<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> CachedMaps<T, N> {
    /// Construct self
    fn new(cfg: &ContextFreeGrammar<T, N>) -> Self {
        let first = compute_first(cfg);

        let generates_empty = first
            .iter()
            .map(|(n, set)| (n.clone(), set.contains(&None)))
            .collect();

        let follow = compute_follow(cfg, &first);

        let predict_sets = compute_predict_sets(cfg, &first, &follow);

        let first = first
            .into_iter()
            .map(|(n, rhs)| (n, rhs.into_iter().flatten().collect()))
            .collect();

        Self {
            first_sets: first,
            follow_sets: follow,
            generates_empty,
            predict_sets,
        }
    }

    /// Determines whether a symbol can generate the empty string. Terminals never do this.
    fn symbol_generates_empty(&self, s: &Symbol<T, N>) -> bool {
        match s {
            Symbol::Nonterminal(n) => self.nonterminal_generates_empty(n),
            Symbol::Terminal(_) => false,
        }
    }

    /// Determines whether a nonterminal can generate the empty string.
    fn nonterminal_generates_empty(&self, n: &N) -> bool {
        *self.generates_empty.get(n).unwrap()
    }

    /// Determines the result of first() for a right-hand-side of a production rule.
    fn first_of_rhs(&self, rhs: &[Symbol<T, N>]) -> HashSet<T> {
        let mut set = HashSet::new();

        for s in rhs {
            match s {
                Symbol::Nonterminal(n) => {
                    set.extend(self.first_of_nonterminal(n).clone());
                }
                Symbol::Terminal(t) => {
                    set.insert(t.clone());
                }
            }
            // repeat until a non-empty symbol has been reached
            if !self.symbol_generates_empty(s) {
                break;
            }
        }

        set
    }

    /// Determines the result of first() for a nonterminal, using all of its production rules.
    fn first_of_nonterminal(&self, n: &N) -> &HashSet<T> {
        self.first_sets.get(n).unwrap()
    }

    /// Determines the result of follow() for a nonterminal, using all production rules of the grammar.
    fn follow_of_nonterminal(&self, n: &N) -> &HashSet<T> {
        self.follow_sets.get(n).unwrap()
    }
}
