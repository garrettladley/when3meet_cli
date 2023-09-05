pub mod errors;
pub mod model;
pub mod parse;

pub use model::{fold, Person, Slot};
pub use parse::parse_when2meet;
