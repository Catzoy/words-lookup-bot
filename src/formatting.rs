use crate::stands4::entities::{AbbreviationDefinition, PhraseDefinition, WordDefinition};

pub trait LookupFormatter<T> {
    fn visit_word(&mut self, i: usize, def: &WordDefinition);
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition);
    fn visit_abbreviations(&mut self, i: usize, category: &str, defs: &Vec<&AbbreviationDefinition>);
    fn append_link(&mut self, link: String);

    fn build(self) -> T;
}
