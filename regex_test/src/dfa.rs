use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub use crate::Error;
type Result<T, S, A> = core::result::Result<T, Error<S, A>>;

#[derive(Debug)]
/// Deterministic Finite Autamota.
///
/// A special case of NFA in which:
///  - no state has an epsillon transition
///  - for each state s and symbol a, there is at most one edge labeled a leaving s.
pub struct Dfa<S, A> {
    edges: HashMap<(S, A), S>,
    initial: S,
    accepting: HashSet<S>,
    alphabet: HashSet<A>,
}

impl<S, A> Dfa<S, A>
where
    S: Copy + Eq + Hash,
    A: Copy + Eq + Hash,
{
    pub fn new(
        states: HashSet<S>,
        alphabet: HashSet<A>,
        edges: HashMap<(S, A), S>,
        initial: S,
        accepting: HashSet<S>,
    ) -> Result<Self, S, A> {
        // check if table includes invalid states or symbols
        for ((state, symbol), edge) in edges.iter() {
            if !states.contains(state) {
                return Err(Error::UnknownState(*state));
            }
            if !alphabet.contains(symbol) {
                return Err(Error::UnknownSymbol(*symbol));
            }
            if !states.contains(edge) {
                return Err(Error::UnknownState(*edge));
            }
        }

        // check if all accepting states are in the set of states
        if let Some(state) = (&accepting - &states).iter().next() {
            return Err(Error::UnknownAcceptingState(*state));
        }

        // check if the initial state is in the set of states
        if !states.contains(&initial) {
            return Err(Error::UnknownInitialState(initial));
        }

        Ok(Self {
            edges,
            initial,
            accepting,
            alphabet,
        })
    }

    pub fn edge(&self, s: &S, c: &A) -> Option<&S> {
        self.edges.get(&(*s, *c))
    }

    pub fn simulate_dfa<C>(&self, c: C) -> Result<bool, S, A>
    where
        C: IntoIterator<Item = A>,
    {
        let mut d = &self.initial;
        for c in c {
            if !self.alphabet.contains(&c) {
                return Err(Error::UnknownSymbol(c));
            }

            if let Some(edge) = self.edge(d, &c) {
                d = edge;
            } else {
                return Ok(false);
            }
        }
        Ok(self.accepting.contains(d))
    }
}
