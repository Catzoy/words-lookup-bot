pub mod inlines;
pub mod word_lookup;
pub mod phrase_lookup;
pub mod debouncer;
mod formatting;
mod suggestions;

use debouncer::*;
use inlines::*;
use phrase_lookup::*;
use suggestions::*;
use word_lookup::*;
