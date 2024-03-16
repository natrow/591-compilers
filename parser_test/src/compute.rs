//! Compute the first and follow sets of a CFG.

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::cfg::*;

/// Compute the first sets of a context-free grammar
pub fn compute_first<T, N>(cfg: &ContextFreeGrammar<T, N>) -> HashMap<N, HashSet<Option<T>>>
where
    T: Eq + Hash + Clone,
    N: Eq + Hash + Clone,
{
    // 1. initialize every Fi(Ai) with the empty set
    let mut first_sets: HashMap<N, HashSet<Option<T>>> = cfg
        .get_nonterminals()
        .iter()
        .cloned()
        .map(|n| (n, HashSet::new()))
        .collect();

    loop {
        // 2. calculate the first of each nonterminal
        let mut new_first = first_sets.clone();

        for (n, rhs) in cfg
            .get_productions()
            .iter()
            .flat_map(|(n, rhs)| rhs.iter().map(move |rhs| (n, rhs)))
        {
            new_first
                .get_mut(n)
                .unwrap()
                .extend(calculate_first(rhs, &first_sets));
        }

        // 3. repeat until the sets are equal
        if first_sets != new_first {
            first_sets = new_first
        } else {
            break;
        }
    }

    first_sets
}

/// Calculate the first of a string given the first sets computed so far
fn calculate_first<T, N>(
    w: &[Symbol<T, N>],
    fi: &HashMap<N, HashSet<Option<T>>>,
) -> HashSet<Option<T>>
where
    T: Eq + Hash + Clone,
    N: Eq + Hash + Clone,
{
    let mut set = HashSet::new();

    if w.is_empty() {
        set.insert(None);
    } else {
        match &w[0] {
            Symbol::Terminal(a) => {
                set.insert(Some(a.clone()));
            }
            Symbol::Nonterminal(n) => {
                let mut fi_n = fi.get(n).unwrap().clone();

                if !fi_n.contains(&None) {
                    set.extend(fi_n)
                } else {
                    fi_n.remove(&None);
                    set.extend(fi_n);
                    set.extend(calculate_first(&w[1..], fi))
                }
            }
        }
    }

    set
}

/// Compute the follow sets of a context-free grammar
pub fn compute_follow<T, N>(
    cfg: &ContextFreeGrammar<T, N>,
    fi: &HashMap<N, HashSet<Option<T>>>,
) -> HashMap<N, HashSet<T>>
where
    T: Eq + Hash + Clone,
    N: Eq + Hash + Clone,
{
    // 1. initialize every Fo(Ai) with the empty set
    let mut follow_sets: HashMap<N, HashSet<T>> = cfg
        .get_nonterminals()
        .iter()
        .cloned()
        .map(|n| (n, HashSet::new()))
        .collect();

    // 2. calculate the follow of each nonterminal
    loop {
        let mut new_follow = follow_sets.clone();
        for n in cfg.get_nonterminals() {
            for r in cfg
                .get_productions()
                .iter()
                .flat_map(|(n, rhs)| rhs.iter().map(move |rhs| (n, rhs)))
            {
                new_follow
                    .get_mut(n)
                    .unwrap()
                    .extend(calculate_follow(n, r, fi, &follow_sets));
            }
        }

        // 3. repeat until the sets are equal
        if follow_sets != new_follow {
            follow_sets = new_follow
        } else {
            break;
        }
    }

    follow_sets
}

/// Calculate the follow of N given the rhs of a rule
fn calculate_follow<T, N>(
    n: &N,
    r: (&N, &Vec<Symbol<T, N>>),
    fi: &HashMap<N, HashSet<Option<T>>>,
    fo: &HashMap<N, HashSet<T>>,
) -> HashSet<T>
where
    T: Eq + Hash + Clone,
    N: Eq + Hash + Clone,
{
    let mut set = HashSet::new();

    // iterate through rhs and find all indices where the symbol is equal to n
    let indices =
        r.1.iter()
            .enumerate()
            .filter_map(|(i, s)| if s == n { Some(i) } else { None });

    // iterate through indices and calculate
    for i in indices {
        let first = calculate_first(&r.1[i + 1..], fi);
        set.extend(first.iter().flatten().cloned());

        if first.contains(&None) {
            set.extend(fo.get(r.0).unwrap().iter().cloned());
        }
    }

    set
}

/// Compute the predict sets of the CFG
pub fn compute_predict_sets<T, N>(
    cfg: &ContextFreeGrammar<T, N>,
    fi: &HashMap<N, HashSet<Option<T>>>,
    fo: &HashMap<N, HashSet<T>>,
) -> HashMap<N, HashSet<T>>
where
    T: Eq + Hash + Clone,
    N: Eq + Hash + Clone,
{
    cfg.get_nonterminals()
        .iter()
        .map(|n| {
            if !fi.get(n).unwrap().contains(&None) {
                (
                    n.clone(),
                    fi.get(n).unwrap().iter().flatten().cloned().collect(),
                )
            } else {
                (
                    n.clone(),
                    fi.get(n)
                        .unwrap()
                        .iter()
                        .flatten()
                        .chain(fo.get(n).unwrap())
                        .cloned()
                        .collect(),
                )
            }
        })
        .collect()
}
