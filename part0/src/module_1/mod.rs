// in Rust, separate files are required to be different modules

mod file_1;
mod file_2;

// however, their (public) contents can be re-exported

pub use file_1::*;
pub use file_2::*;

// due to Rust's privacy system, this effectively allows us to break
// a module down into smaller files
