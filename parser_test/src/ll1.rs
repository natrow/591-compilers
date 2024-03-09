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

use log::{debug, trace};

use crate::cfg::{ContextFreeGrammar, Productions, Symbol};

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
            let first = memoize.first_of_rhs(productions, rhs, &mut [n].into());
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
        if memoize.nonterminal_generates_empty(productions, n, &mut [n].into()) {
            let first = memoize.first_of_nonterminal(productions, n, &mut [n].into());
            let follow = memoize.follow_of_nonterminal(productions, n, &mut [n].into());

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
            let set = memoize.first_of_nonterminal(productions, n, &mut [n].into());
            debug!("FIRST({:?}) = {:?}", n, set);
        }

        debug!("Finished calculating first sets!");

        // calculate follow sets
        for n in cfg.get_nonterminals() {
            let set = memoize.follow_of_nonterminal(productions, n, &mut [n].into());
            debug!("FOLLOW({:?}) = {:?}", n, set);
        }

        debug!("Finished calculating follow sets!");

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
struct Memoize<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> {
    /// Memoized result of the first() function on a nonterminal
    first: HashMap<N, HashSet<T>>,
    /// Memoized result of the follow() function on a nonterminal
    follow: HashMap<N, HashSet<T>>,
    /// Whether or not a nonterminal can generate the empty string
    generates_empty: HashMap<N, bool>,
}

impl<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> Default for Memoize<T, N> {
    fn default() -> Self {
        Self {
            first: Default::default(),
            follow: Default::default(),
            generates_empty: Default::default(),
        }
    }
}

impl<T: Eq + Hash + Clone + Debug, N: Eq + Hash + Clone + Debug> Memoize<T, N> {
    /// Determines whether a symbol can generate the empty string. Terminals never do this.
    fn symbol_generates_empty<'a>(
        &mut self,
        productions: &'a Productions<T, N>,
        s: &'a Symbol<T, N>,
        call_stack: &mut HashSet<&'a N>,
    ) -> bool {
        match s {
            Symbol::Nonterminal(n) => {
                assert!(call_stack.insert(n), "cycle detected");
                let result = self.nonterminal_generates_empty(productions, n, call_stack);
                call_stack.remove(n);
                result
            }
            Symbol::Terminal(_) => false,
        }
    }

    /// Determines whether a nonterminal can generate the empty string.
    fn nonterminal_generates_empty<'a>(
        &mut self,
        productions: &'a Productions<T, N>,
        n: &'a N,
        call_stack: &mut HashSet<&'a N>,
    ) -> bool {
        // 1. Already in the set
        if let Some(v) = self.generates_empty.get(n) {
            return *v;
        }

        trace!("Calculating whether {:?} generates the empty string...", n);

        // 2. A production contains the empty string
        if productions.get(n).unwrap().iter().any(|v| v.is_empty()) {
            self.generates_empty.insert(n.clone(), true);
            trace!("true");
            return true;
        }

        // 3. A production can generate the empty string
        for rhs in productions.get(n).unwrap().iter() {
            if rhs
                .iter()
                .all(|s| self.symbol_generates_empty(productions, s, call_stack))
            {
                self.generates_empty.insert(n.clone(), true);
                trace!("true");
                return true;
            }
        }

        self.generates_empty.insert(n.clone(), false);
        trace!("false");
        false
    }

    /// Determines the result of first() for a symbol. Terminals always return themselves.
    fn first_of_symbol<'a>(
        &mut self,
        productions: &'a Productions<T, N>,
        s: &'a Symbol<T, N>,
        call_stack: &mut HashSet<&'a N>,
    ) -> HashSet<T> {
        match s {
            Symbol::Nonterminal(n) => {
                assert!(call_stack.insert(n), "cycle detected");
                let res = self.first_of_nonterminal(productions, n, call_stack);
                call_stack.remove(n);
                res
            }
            Symbol::Terminal(t) => HashSet::from([t.clone()]),
        }
    }

    /// Determines the result of first() for a right-hand-side of a production rule.
    fn first_of_rhs<'a>(
        &mut self,
        productions: &'a Productions<T, N>,
        rhs: &'a [Symbol<T, N>],
        call_stack: &mut HashSet<&'a N>,
    ) -> HashSet<T> {
        let mut set = HashSet::new();

        for s in rhs {
            set.extend(self.first_of_symbol(productions, s, call_stack));
            // repeat until a non-empty symbol has been reached
            if !self.symbol_generates_empty(productions, s, &mut HashSet::new()) {
                break;
            }
        }

        set
    }

    /// Determines the result of first() for a nonterminal, using all of its production rules.
    fn first_of_nonterminal<'a>(
        &mut self,
        productions: &'a Productions<T, N>,
        n: &'a N,
        call_stack: &mut HashSet<&'a N>,
    ) -> HashSet<T> {
        // if the first set is already calculated, return it
        if let Some(v) = self.first.get(n) {
            return v.clone();
        }

        trace!("Calculating first of {:?}...", n);

        // otherwise calculate the set
        let mut set = HashSet::new();

        // expand each production
        for rhs in productions.get(n).unwrap() {
            set.extend(self.first_of_rhs(productions, rhs, call_stack));
        }

        self.first.insert(n.clone(), set.clone());

        trace!("{:?}", &set);

        set
    }

    /// Determines the result of follow() for a nonterminal, using all production rules of the grammar.
    fn follow_of_nonterminal<'a>(
        &mut self,
        productions: &'a Productions<T, N>,
        n: &'a N,
        call_stack: &mut HashSet<&'a N>,
    ) -> HashSet<T> {
        // if the follow set is already calculated, return it
        if let Some(v) = self.follow.get(n) {
            return v.clone();
        }

        trace!("Calculating follow of {:?}...", n);

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
                        set.extend(self.first_of_rhs(
                            productions,
                            &p.1[i + 1..],
                            &mut HashSet::new(),
                        ));

                        // if the rest of the rhs can generate the empty string, append the follow of the lhs of the production
                        if !call_stack.contains(p.0)
                            && p.1[i + 1..].iter().all(|s| {
                                self.symbol_generates_empty(productions, s, &mut HashSet::new())
                            })
                        {
                            call_stack.insert(p.0);

                            set.extend(self.follow_of_nonterminal(productions, p.0, call_stack));

                            call_stack.remove(p.0);
                        }
                    } else if !call_stack.contains(p.0) {
                        call_stack.insert(p.0);

                        // otherwise append the follow of the lhs of the production
                        set.extend(self.follow_of_nonterminal(productions, p.0, call_stack));

                        call_stack.remove(p.0);
                    }
                }
            }
        }

        self.follow.insert(n.clone(), set.clone());

        trace!("{:?}", &set);

        set
    }
}
