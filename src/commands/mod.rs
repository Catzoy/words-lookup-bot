pub mod start;
pub mod command;
pub mod unknown;
pub mod teapot;
pub mod word_lookup;
pub mod phrase_lookup;
pub mod help;
mod formatting;

pub use command::*;
pub use formatting::*;
pub use help::*;
pub use phrase_lookup::*;
pub use start::*;
pub use teapot::*;
pub use unknown::*;
pub use word_lookup::*;
