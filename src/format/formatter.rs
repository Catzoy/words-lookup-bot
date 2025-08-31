use crate::stands4::{AbbreviationDefinition, PhraseDefinition, VecAbbreviationsExt, WordDefinition};
use crate::urban::UrbanDefinition;

pub trait LinkProvider {
    fn word_link(&self, word: &str) -> String;
    fn abbr_link(&self, abbr: &str) -> String;
    fn phrase_link(&self, phrase: &str) -> String;

    fn urban_link(&self, term: &str) -> String;
}

pub trait LookupFormatter<T> {
    fn link_provider(&self) -> &dyn LinkProvider;
    fn visit_word(&mut self, i: usize, def: &WordDefinition);
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition);
    fn visit_abbreviations(&mut self, i: usize, category: &str, defs: &Vec<&AbbreviationDefinition>);
    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition);
    fn append_title(&mut self, title: String);
    fn append_link(&mut self, link: String);
    fn build(self) -> T;
}

pub fn compose_word_defs<R, Formatter: LookupFormatter<R>>(
    mut formatter: Formatter,
    word: &str,
    defs: &Vec<WordDefinition>,
) -> R {
    formatter.append_title(format!("Found {} definitions", defs.len()));

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_word(i, def);
    }
    if defs.len() > 5 {
        formatter.append_link(formatter.link_provider().word_link(word))
    }
    formatter.build()
}

pub fn compose_urban_defs<R, Formatter: LookupFormatter<R>>(
    mut formatter: Formatter,
    word: &str,
    defs: &Vec<UrbanDefinition>,
) -> R {
    formatter.append_title(format!("Found {} definitions from Urban Dictionary", defs.len()));

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_urban_definition(i, def);
    }
    if defs.len() > 5 {
        formatter.append_link(formatter.link_provider().urban_link(word))
    }
    formatter.build()
}

pub fn compose_abbr_defs<R, Formatter: LookupFormatter<R>>(
    mut formatter: Formatter,
    word: &str,
    defs: &Vec<AbbreviationDefinition>,
) -> R {
    formatter.append_title(format!("Found {} definitions", defs.len()));

    let categorized = defs.categorized();
    for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
        formatter.visit_abbreviations(i, category, defs);
    }
    if categorized.len() > 5 {
        formatter.append_link(formatter.link_provider().abbr_link(word))
    }
    formatter.build()
}

pub fn compose_words_with_abbrs<R, Formatter: LookupFormatter<R>>(
    mut formatter: Formatter,
    word: &str,
    words: &Vec<WordDefinition>,
    abbrs: &Vec<AbbreviationDefinition>,
) -> R {
    formatter.append_title(format!("Found {} definitions", words.len()));

    for (i, def) in words.iter().take(5).enumerate() {
        formatter.visit_word(i, def);
    }
    if words.len() > 5 {
        formatter.append_link(formatter.link_provider().word_link(word))
    }

    formatter.append_title(format!("Found {} abbreviations", abbrs.len()));

    let categorized = abbrs.categorized();
    for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
        formatter.visit_abbreviations(i, category, defs);
    }
    if categorized.len() > 5 {
        formatter.append_link(formatter.link_provider().abbr_link(word))
    }

    formatter.build()
}

pub fn compose_phrase_defs<R, Formatter: LookupFormatter<R>>(
    mut formatter: Formatter,
    phrase: &str,
    defs: &Vec<PhraseDefinition>,
) -> R {
    formatter.append_title(format!("Found {} definitions", defs.len()));

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_phrase(i, def);
    }
    if defs.len() > 5 {
        formatter.append_link(formatter.link_provider().phrase_link(phrase));
    }

    formatter.build()
}