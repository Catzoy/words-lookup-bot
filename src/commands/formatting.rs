use crate::bloc::formatting::SynAntFormatterExt;
use crate::format::{as_in, meaning, ToEscaped};
use crate::{
    format::{LinksProvider, LookupFormatter, StringBuilderExt},
    stands4::entities::{AbbreviationDefinition, PhraseDefinition, WordDefinition},
    stands4::SynAntDefinitions,
    urban::UrbanDefinition,
};
use std::ops::Not;

#[derive(Default)]
pub struct FullMessageFormatter {
    builder: string_builder::Builder,
    link_provider: LinksProvider,
}

impl LookupFormatter for FullMessageFormatter {
    type Error = std::string::FromUtf8Error;
    type Value = String;
    /// Returns the default message used when no definitions are found.
    ///
    /// # Returns
    ///
    /// A `String` containing the literal message "Found 0 definitions".
    ///
    /// # Examples
    ///
    /// ```
    /// let msg = FullMessageFormatter::on_empty();
    /// assert_eq!(msg, "Found 0 definitions");
    /// ```
    fn on_empty() -> Self::Value {
        "Found 0 definitions".to_string()
    }
    /// Access the formatter's links provider.
    ///
    /// Returns a reference to the internal `LinksProvider`.
    ///
    /// # Examples
    ///
    /// ```
    /// let fmt = FullMessageFormatter::default();
    /// let _links = fmt.link_provider();
    /// ```
    fn link_provider(&self) -> &LinksProvider {
        &self.link_provider
    }

    /// Appends a formatted word entry (index, term, part of speech, meaning, and optional example) to the formatter's internal builder.
    ///
    /// The appended entry includes the 1-based index, the escaped term, the part of speech (or `"?"` if empty),
    /// the formatted meaning on its own line, and an optional formatted example if present. A blank line is appended after each entry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut fmt = FullMessageFormatter::default();
    /// let word = WordDefinition {
    ///     term: "rust".into(),
    ///     part_of_speech: "noun".into(),
    ///     definition: "A systems programming language.".into(),
    ///     example: "I write code in Rust.".into(),
    ///     ..Default::default()
    /// };
    /// fmt.visit_word(0, &word);
    /// // The internal builder now contains a formatted entry for "rust".
    /// ```
    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let def = def.to_escaped();
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        self.builder.append(format!(
            "\\#{} \\- {} \\({}\\)\n",
            i + 1,
            def.term,
            part_of_speech
        ));
        self.builder.appendl(meaning(&def.definition));
        if def.example.is_empty().not() {
            self.builder.appendl(as_in(&def.example));
        }
        self.builder.append("\n");
    }

    /// Appends a formatted phrase entry (index, term, explanation and optional example) to the internal message builder.
    ///
    /// The entry is numbered using `i + 1`, the phrase term is escaped, the explanation is formatted via `meaning`, and
    /// a non-empty example is formatted via `as_in` and appended on its own line.
    ///
    /// # Examples
    ///
    /// ```
    /// # use your_crate::{FullMessageFormatter, PhraseDefinition};
    /// let mut fmt = FullMessageFormatter::default();
    /// let def = PhraseDefinition {
    ///     term: "break the ice".into(),
    ///     explanation: "to initiate social interaction".into(),
    ///     example: "He told a joke to break the ice.".into(),
    /// };
    /// fmt.visit_phrase(0, &def);
    /// let out = fmt.build().unwrap();
    /// assert!(out.contains("1"));
    /// assert!(out.contains("break the ice"));
    /// assert!(out.contains("to initiate social interaction"));
    /// assert!(out.contains("break the ice")); // example present
    /// ```
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        let def = def.to_escaped();
        self.builder
            .append(format!("\\#{} \\- {}\n", i + 1, def.term));
        self.builder.appendl(meaning(&def.explanation));
        if def.example.is_empty().not() {
            self.builder.appendl(as_in(&def.example));
        }
        self.builder.append("\n");
    }

    /// Appends a formatted abbreviations block for the given category and definitions to the builder.
    ///
    /// The method formats a header "#<index> in [<category>] stands for:", joins the provided
    /// abbreviation definitions by their `definition` field separated with ", ", and appends a blank line.
    /// If `category` is empty, it is treated as "uncategorized".
    ///
    /// # Parameters
    ///
    /// - `i`: zero-based index used to number the block (displayed as `i + 1`).
    /// - `category`: name of the abbreviations category; empty string becomes `"uncategorized"`.
    /// - `defs`: list of references to `AbbreviationDefinition` values; each is converted with `to_escaped()`
    ///   before its `definition` field is used.
    ///
    /// # Examples
    ///
    /// ```
    /// struct AbbreviationDefinition { definition: String }
    /// impl AbbreviationDefinition {
    ///     fn to_escaped(&self) -> Self { Self { definition: self.definition.clone() } }
    /// }
    ///
    /// struct MockBuilder { out: String }
    /// impl MockBuilder {
    ///     fn new() -> Self { Self { out: String::new() } }
    ///     fn append(&mut self, s: String) { self.out.push_str(&s); }
    ///     fn appendl(&mut self, s: &str) { self.out.push_str(s); }
    ///     fn join<T, F, G>(&mut self, items: &Vec<T>, mut f: F, mut sep: G)
    ///         where F: FnMut(&mut Self, &T), G: FnMut(&mut Self)
    ///     {
    ///         let mut first = true;
    ///         for item in items {
    ///             if !first { sep(self); }
    ///             f(self, item);
    ///             first = false;
    ///         }
    ///     }
    /// }
    ///
    /// struct FullMessageFormatter { builder: MockBuilder }
    /// impl FullMessageFormatter {
    ///     fn new() -> Self { Self { builder: MockBuilder::new() } }
    ///     fn visit_abbreviations(
    ///         &mut self,
    ///         i: usize,
    ///         category: &str,
    ///         defs: &Vec<&AbbreviationDefinition>,
    ///     ) {
    ///         let defs = defs.iter().map(|d| d.to_escaped()).collect::<Vec<_>>();
    ///         let category = if category.is_empty() { "uncategorized".to_string() } else { category.to_string() };
    ///         self.builder.append(format!("\\#{} in \\[{}\\] stands for: \n", i + 1, category));
    ///         self.builder.join(
    ///             &defs,
    ///             |builder, def| builder.append(def.definition.clone()),
    ///             |builder| builder.appendl(", "),
    ///         );
    ///         self.builder.append("\n".to_string());
    ///     }
    /// }
    ///
    /// let a = AbbreviationDefinition { definition: "Alpha".to_string() };
    /// let b = AbbreviationDefinition { definition: "Beta".to_string() };
    /// let mut fmt = FullMessageFormatter::new();
    /// fmt.visit_abbreviations(0, "", &vec![&a, &b]);
    /// assert!(fmt.builder.out.contains("uncategorized"));
    /// assert!(fmt.builder.out.contains("Alpha"));
    /// assert!(fmt.builder.out.contains("Beta"));
    /// ```
    fn visit_abbreviations(
        &mut self,
        i: usize,
        category: &str,
        defs: &Vec<&AbbreviationDefinition>,
    ) {
        let defs = defs.iter().map(|d| d.to_escaped()).collect();
        let category = match category.len() {
            0 => "uncategorized".to_string(),
            _ => category.to_string(),
        };

        self.builder
            .append(format!("\\#{} in \\[{}\\] stands for: \n", i + 1, category));
        self.builder.join(
            &defs,
            |builder, def| builder.append(def.definition.as_str()),
            |builder| builder.appendl(", "),
        );
        self.builder.append("\n");
    }

    /// Appends a formatted synonym/antonym entry for a definition into the internal builder.
    ///
    /// The entry consists of a numbered header with the escaped term, the formatted meaning,
    /// followed by any synonyms and antonyms; if none are present a fallback message is appended.
    ///
    /// # Parameters
    ///
    /// - `i`: zero-based index used for numbering the entry.
    /// - `def`: the synonym/antonym definition to format and append (will be escaped).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = FullMessageFormatter::default();
    /// let def = SynAntDefinitions {
    ///     term: "test".to_string(),
    ///     definition: "an exam".to_string(),
    ///     synonyms: vec![],
    ///     antonyms: vec![],
    /// };
    /// fmt.visit_syn_ant(0, &def);
    /// let out = fmt.build().unwrap();
    /// assert!(out.contains("#1 - test"));
    /// assert!(out.contains("an exam"));
    /// ```
    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions) {
        let def = def.to_escaped();
        self.builder
            .append(format!("\\#{} \\- {}\n", i + 1, def.term));
        self.builder.appendl(meaning(&def.definition));
        Self::push_syn_ant(&mut self.builder, &def, || {
            "Surprisingly, there are no other ways to express neither something similar, nor the opposite!".to_string()
        });
        self.builder.append("\n");
    }

    /// Appends an UrbanDictionary-style definition entry for `def` to the internal builder.
    ///
    /// The entry consists of a numbered header (`#<index> - <word>`), the formatted meaning,
    /// an optional formatted example if present, and a trailing blank line.
    ///
    /// # Parameters
    ///
    /// - `i`: zero-based index of the definition (displayed as `i + 1`).
    /// - `def`: the urban definition to format and append; it is escaped prior to formatting.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = FullMessageFormatter::default();
    /// let def = UrbanDefinition {
    ///     word: "yeet".into(),
    ///     meaning: "to throw forcefully".into(),
    ///     example: Some("He yeeted the ball across the yard.".into()),
    /// };
    /// fmt.visit_urban_definition(0, &def);
    /// let out = fmt.build().unwrap();
    /// assert!(out.contains("#1 - yeet"));
    /// assert!(out.contains("to throw forcefully"));
    /// assert!(out.contains("He yeeted the ball"));
    /// ```
    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        let def = def.to_escaped();
        self.builder
            .append(format!("\\#{} \\- {}\n", i + 1, def.word));
        self.builder.appendl(meaning(&def.meaning));
        if let Some(example) = &def.example {
            self.builder.appendl(as_in(example));
        }
        self.builder.append("\n");
    }

    fn append_title(&mut self, title: String) {
        self.builder.append(format!("{}\n\n", title));
    }

    fn append_link(&mut self, link: String) {
        self.builder
            .append(format!("Check out other definitions at {}\n\n", link));
    }

    fn build(self) -> Result<String, std::string::FromUtf8Error> {
        self.builder.string()
    }
}