use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use log::{debug, trace, warn};

use crate::dfa::Dfa;
pub use crate::Error;
type Result<T, S, A> = core::result::Result<T, Error<S, A>>;

#[derive(Debug)]
/// Non-deterministic Finite Autamota.
///
/// A mathematical model that consists of:
///  - A set of states S
///  - A set of input symbols Sigma
///  - A transition function "move" that maps state-symbol pairs to sets of states
///  - A state s0 designated as the start (or initial) state
///  - A set of states F designated as accepting (or final) states
pub struct Nfa<S, A> {
    edges: HashMap<(S, Option<A>), HashSet<S>>,
    initial: S,
    accepting: HashSet<S>,
    alphabet: HashSet<A>,
}

impl<S, A> Nfa<S, A>
where
    S: Copy + Eq + Hash + Debug,
    A: Copy + Eq + Hash + Debug,
{
    /// A constructor which validates each input to garauntee invariance of the NFA.
    pub fn new(
        states: HashSet<S>,
        alphabet: HashSet<A>,
        edges: HashMap<(S, Option<A>), HashSet<S>>,
        initial: S,
        accepting: HashSet<S>,
    ) -> Result<Self, S, A> {
        // validate each input to garauntee invariance

        // check if table includes invalid states or symbols
        for ((state, symbol), edge) in edges.iter() {
            if !states.contains(state) {
                return Err(Error::UnknownState(*state));
            }
            if let Some(symbol) = symbol {
                if !alphabet.contains(symbol) {
                    return Err(Error::UnknownSymbol(*symbol));
                }
            }
            for state in edge.iter() {
                if !states.contains(state) {
                    return Err(Error::UnknownState(*state));
                }
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

    /// Returns the set of resulting states for a given state symbol pair
    pub fn edge(&self, s: &S, c: &Option<A>) -> HashSet<S> {
        self.edges.get(&(*s, *c)).cloned().unwrap_or_default()
    }

    /// Returns the union of the edges of the given set of states and a symbol
    pub fn union_edge(&self, s: &HashSet<S>, c: &Option<A>) -> HashSet<S> {
        trace!("union of edges {:?}, {:?}", s, c);
        s.iter()
            .map(|s| self.edge(s, c))
            .fold(HashSet::new(), |mut union, s| {
                union.extend(s);
                union
            })
    }

    /// Returns the set of all states reachable through epsillon edges alone.
    pub fn e_closure(&self, s: &HashSet<S>) -> HashSet<S> {
        trace!("epsilon closure of {:?}", s);
        let mut t = s.clone();
        let mut tp;
        loop {
            tp = t.clone();
            t.extend(self.union_edge(&tp, &None));
            if t == tp {
                break;
            }
        }
        trace!("got {:?}", t);
        t
    }

    /// Returns all states reachable reachable through epsillon edges and an input symbol.
    pub fn dfa_edge(&self, d: &HashSet<S>, c: &A) -> HashSet<S> {
        trace!("finding dfa edge {:?}, {:?}", d, c);
        let r = self.e_closure(&self.union_edge(d, &Some(*c)));
        trace!("got {:?}", r);
        r
    }

    pub fn simulate_nfa<C>(&self, c: C) -> Result<HashSet<S>, S, A>
    where
        C: IntoIterator<Item = A>,
    {
        debug!("simulating nfa...");
        let mut d = self.e_closure(&[self.initial].into());

        for c in c {
            debug!("state: {:?}, input: {:?}", d, c);
            if !self.alphabet.contains(&c) {
                warn!("got unknown symbol {:?}, done", c);
                return Err(Error::UnknownSymbol(c));
            }
            d = self.dfa_edge(&d, &c)
        }

        debug!("done, final state: {:?}", d);

        Ok(&d & &self.accepting)
    }

    pub fn construct_subsets(&self) -> Result<Dfa<usize, A>, usize, A> {
        let mut states = vec![self.e_closure(&[self.initial].into())];
        let mut trans = HashMap::new();

        let mut j = 0;

        while j < states.len() {
            for c in self.alphabet.iter() {
                let e = self.dfa_edge(&states[j], c);

                // don't add error states
                if e.is_empty() {
                    continue;
                }

                if let Some(i) = states.iter().position(|v| *v == e) {
                    trans.insert((j, *c), i);
                    trace!("inserting {} -> {} on {:?}", j, i, c);
                } else {
                    trace!("creating state {:?}", e);
                    states.push(e);
                    trace!("inserting {} -> {} on {:?}", j, states.len() - 1, c);
                    trans.insert((j, *c), states.len() - 1);
                }
            }
            j += 1;
        }

        debug!("created DFA with {} states", states.len());
        for (k, v) in trans.iter() {
            debug!("{} -> {} on {:?}", k.0, v, k.1);
        }

        let accepting = states
            .iter()
            .enumerate()
            .filter(|(_, s)| !s.is_disjoint(&self.accepting))
            .map(|(i, _)| i)
            .collect();

        let states = (0..states.len()).collect();

        Dfa::new(states, self.alphabet.clone(), trans, 0, accepting)
    }
}
