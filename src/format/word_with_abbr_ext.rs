use crate::format::formatter::{compose_abbr_defs, compose_word_defs, compose_words_with_abbrs, LookupFormatter};
use crate::stands4::{AbbreviationDefinition, WordDefinition};

pub fn compose_word_with_abbrs_determined<V, E, Formatter: LookupFormatter<Result<V, E>>>(
    formatter: Formatter,
    word: &str,
    results: &(anyhow::Result<Vec<WordDefinition>>, anyhow::Result<Vec<AbbreviationDefinition>>),
    on_empty: fn() -> V,
) -> Result<V, E> {
    match results {
        (Ok(words), Ok(abbrs)) =>
            match (words.len(), abbrs.len()) {
                (0, 0) => Ok(on_empty()),
                (0, _) => compose_abbr_defs(formatter, word, abbrs),
                (_, 0) => compose_word_defs(formatter, word, words),
                (_, _) => compose_words_with_abbrs(formatter, word, words, abbrs)
            }

        (Ok(words), _) =>
            compose_word_defs(formatter, word, words),

        (_, Ok(abbrs)) =>
            compose_abbr_defs(formatter, word, abbrs),

        (Err(_), Err(_)) =>
            Ok(on_empty()),
    }
}