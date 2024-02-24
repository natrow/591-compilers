//! Test crate for experimenting with LL(1) grammars and parsers.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod cfg;
pub mod ll1;

#[cfg(test)]
mod test {
    use log::debug;

    use crate::{
        cfg::{ContextFreeGrammar, Symbol},
        ll1::LL1,
    };

    impl From<&'static str> for Symbol<char, &'static str> {
        fn from(value: &'static str) -> Self {
            Self::Nonterminal(value)
        }
    }

    impl From<char> for Symbol<char, &'static str> {
        fn from(value: char) -> Self {
            Self::Terminal(value)
        }
    }

    #[test]
    fn is_ll1() {
        env_logger::try_init().ok();

        debug!("Evaluating grammar:");
        debug!("S -> E0 $");
        debug!("E0 -> E1 E0'");
        debug!("E0' -> + E1 E0' | - E1 E0' | epsilon");
        debug!("E1 -> E2 E1'");
        debug!("E1' -> * E2 E1' | / E2 E1' | epsilon");
        debug!("E2 -> n | ( E0 )");

        let nonterminals = ["S", "E0", "E0'", "E1", "E1'", "E2"].into();
        let terminals = ['n', '(', ')', '+', '-', '*', '/', '$'].into();

        let s = [vec!["E0".into(), '$'.into()]].into();

        let e0 = [vec!["E1".into(), "E0'".into()]].into();
        let e0p = [
            vec!['+'.into(), "E1".into(), "E0'".into()],
            vec!['-'.into(), "E1".into(), "E0'".into()],
            vec![],
        ]
        .into();

        let e1 = [vec!["E2".into(), "E1'".into()]].into();
        let e1p = [
            vec!['*'.into(), "E2".into(), "E1'".into()],
            vec!['/'.into(), "E2".into(), "E1'".into()],
            vec![],
        ]
        .into();

        let e2 = [vec!['n'.into()], vec!['('.into(), "E0".into(), ')'.into()]].into();

        let productions = [
            ("S", s),
            ("E0", e0),
            ("E0'", e0p),
            ("E1", e1),
            ("E1'", e1p),
            ("E2", e2),
        ]
        .into();

        let cfg = ContextFreeGrammar::new(terminals, nonterminals, productions).unwrap();

        debug!("created grammar.");

        let _ll1 = LL1::new(cfg).unwrap();

        debug!("grammar is LL(1)");
    }
}
