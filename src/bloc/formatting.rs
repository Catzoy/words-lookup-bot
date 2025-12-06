use crate::format::{LookupFormatter, StringBuilderExt};
use crate::stands4::SynAntDefinitions;
use string_builder::Builder;

pub trait SynAntFormatterExt {
    fn push_syn_ant(builder: &mut Builder, def: &SynAntDefinitions, on_empty: fn() -> String);
}

impl<L> SynAntFormatterExt for L
where
    L: LookupFormatter,
{
    /// Appends formatted synonym and antonym sections from `def` into `builder`, or appends the `on_empty` fallback when neither are present.
    ///
    /// If `def.synonyms` is non-empty, a "*Synonyms*: " label followed by the word list and a newline is appended.
    /// If `def.antonyms` is non-empty, a "*Antonyms*: " label followed by the word list and a newline is appended.
    /// If both are empty, the string returned by `on_empty()` is appended instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use string_builder::Builder;
    /// use crate::stands4::SynAntDefinitions;
    ///
    /// let mut builder = Builder::new();
    /// let def = SynAntDefinitions { synonyms: vec!["fast".into()], antonyms: vec![] };
    /// push_syn_ant(&mut builder, &def, || "No synonyms or antonyms available.".into());
    /// let output = builder.into_string().unwrap();
    /// assert!(output.contains("Synonyms"));
    /// ```
    fn push_syn_ant(builder: &mut Builder, def: &SynAntDefinitions, on_empty: fn() -> String) {
        let mut has_any = false;

        if !def.synonyms.is_empty() {
            builder.append("*Synonyms*: ");
            builder.list_words(&def.synonyms);
            builder.append("\n");
            has_any = true;
        }

        if !def.antonyms.is_empty() {
            builder.append("*Antonyms*: ");
            builder.list_words(&def.antonyms);
            builder.append("\n");
            has_any = true;
        }

        if !has_any {
            builder.append(on_empty());
        }
    }
}
