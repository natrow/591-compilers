//! Test crate for experimenting with LL(1) grammars and parsers.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod cfg;
mod compute;
pub mod ll1;
pub mod token;

#[cfg(test)]
mod test {
    /// toyc LL(1) test - Nathan's version
    mod toyc_nathan;
    /// toyc LL(1) test - Trevin's version
    mod toyc_trevin;
}
