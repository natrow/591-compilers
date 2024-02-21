//! Regular expressions metalanguage:
//! a (char)
//! | (or)
//! . (any)
//! ^ (not)
//! ? (maybe)
//! * (repeating)
//! + (at least one)
//! () (group)
//! [] (character class)
//! \ (escape metalanguage)
//! - (through)

pub mod dfa;
pub mod nfa;
pub mod parser;
pub mod scanner;

#[derive(Debug, PartialEq, Eq)]
/// Errors that can happen while evaluating a finite automata.
pub enum Error<S, A> {
    /// Found a state which was not in the set of states
    UnknownState(S),
    /// Found a symbol which was not in the alphabet
    UnknownSymbol(A),
    /// Found an accepting state which was not in the set of states
    UnknownAcceptingState(S),
    /// Found an initial state which was not in the set of states
    UnknownInitialState(S),
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::nfa::{Error, Nfa};

    #[test]
    fn test_nfa() {
        env_logger::try_init().ok();

        // (a|b)*abb

        let states = (1..=11).collect();
        let alphabet = ['a', 'b'].into();

        let mut edges = HashMap::new();
        edges.insert((1, None), [2, 8].into());
        edges.insert((2, None), [3, 5].into());
        edges.insert((3, Some('a')), [4].into());
        edges.insert((4, None), [7].into());
        edges.insert((5, Some('b')), [6].into());
        edges.insert((6, None), [7].into());
        edges.insert((7, None), [2, 8].into());
        edges.insert((8, Some('a')), [9].into());
        edges.insert((9, Some('b')), [10].into());
        edges.insert((10, Some('b')), [11].into());

        let initial = 1;
        let accepting = [11].into();

        let nfa = Nfa::new(states, alphabet, edges, initial, accepting).unwrap();

        println!("nfa is {:?}", nfa);

        assert_eq!(
            nfa.simulate_nfa("x".chars()).unwrap_err(),
            Error::UnknownSymbol('x')
        );

        // (a|b)*abb
        assert_eq!(nfa.simulate_nfa("a".chars()).unwrap(), HashSet::new());
        assert_eq!(nfa.simulate_nfa("aabba".chars()).unwrap(), HashSet::new());
        assert_eq!(nfa.simulate_nfa("abb".chars()).unwrap(), [11].into());
    }

    #[test]
    fn test_dfa1() {
        env_logger::try_init().ok();

        // (a|b)*abb

        let states = (1..=11).collect();
        let alphabet = ['a', 'b'].into();

        let mut edges = HashMap::new();
        edges.insert((1, None), [2, 8].into());
        edges.insert((2, None), [3, 5].into());
        edges.insert((3, Some('a')), [4].into());
        edges.insert((4, None), [7].into());
        edges.insert((5, Some('b')), [6].into());
        edges.insert((6, None), [7].into());
        edges.insert((7, None), [2, 8].into());
        edges.insert((8, Some('a')), [9].into());
        edges.insert((9, Some('b')), [10].into());
        edges.insert((10, Some('b')), [11].into());

        let initial = 1;
        let accepting = [11].into();

        let nfa = Nfa::new(states, alphabet, edges, initial, accepting).unwrap();

        let dfa = nfa.construct_subsets().unwrap();

        println!("dfa is {:?}", dfa);
    }

    #[test]
    fn test_dfa2() {
        env_logger::try_init().ok();

        // x?(ab)*

        let states = (1..=8).collect();
        let alphabet = ['x', 'a', 'b'].into();

        let mut edges = HashMap::new();
        edges.insert((1, None), [2].into());
        edges.insert((2, None), [3].into());
        edges.insert((2, Some('x')), [4].into());
        edges.insert((3, None), [5].into());
        edges.insert((4, None), [5].into());
        edges.insert((5, None), [6].into());
        edges.insert((6, Some('a')), [7].into());
        edges.insert((7, Some('b')), [8].into());
        edges.insert((8, None), [6].into());

        let initial = 1;
        let accepting = [6].into();

        let nfa = Nfa::new(states, alphabet, edges, initial, accepting).unwrap();

        let dfa = nfa.construct_subsets().unwrap();

        println!("dfa is {:?}", dfa);
    }
}
