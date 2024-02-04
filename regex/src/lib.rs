pub mod dfa;
pub mod nfa;
pub mod parser;

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::nfa::{Error, Nfa};

    #[test]
    fn test_nfa() {
        env_logger::init();

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
}
