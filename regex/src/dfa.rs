use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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
}

impl<S, A> Dfa<S, A>
where
    S: Clone + Eq + Hash,
    A: Copy + Eq + Hash,
{
    pub fn edge(&self, s: &S, c: &A) -> Option<&S> {
        self.edges.get(&(s.clone(), *c))
    }

    pub fn simulate_dfa<C>(&self, c: C) -> bool
    where
        C: IntoIterator<Item = A>,
    {
        let mut d = self.initial.clone();
        for c in c {
            if let Some(edge) = self.edge(&d, &c) {
                d = edge.clone();
            } else {
                return false;
            }
        }
        self.accepting.contains(&d)
    }
}
