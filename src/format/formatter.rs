use crate::format::LinksProvider;
use crate::stands4::{AbbreviationDefinition, PhraseDefinition, SynAntDefinitions, WordDefinition};
use crate::urban::UrbanDefinition;
use regex::Regex;
use std::fmt::Debug;
use std::ops::Index;
use std::sync::LazyLock;

pub trait LookupFormatter<T> {
    type Error: Debug;
    fn link_provider(&self) -> &LinksProvider;
    fn visit_word(&mut self, i: usize, def: &WordDefinition);
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition);
    fn visit_abbreviations(
        &mut self,
        i: usize,
        category: &str,
        defs: &Vec<&AbbreviationDefinition>,
    );
    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions);
    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition);
    fn append_title(&mut self, title: String);
    fn append_link(&mut self, link: String);
    fn build(self) -> Result<T, Self::Error>;
}

pub trait ToEscaped {
    fn to_escaped(&self) -> Self;
}

impl<T> ToEscaped for Vec<T>
where
    T: ToEscaped,
{
    fn to_escaped(&self) -> Self {
        self.iter().map(|i| i.to_escaped()).collect()
    }
}

impl<T> ToEscaped for Option<T>
where
    T: ToEscaped,
{
    fn to_escaped(&self) -> Self {
        match self {
            None => None,
            Some(it) => Some(it.to_escaped()),
        }
    }
}

impl ToEscaped for String {
    fn to_escaped(&self) -> Self {
        teloxide::utils::markdown::escape(self)
    }
}

static LINE_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.+)\n*").unwrap());

fn lines_of(str: &String) -> Vec<String> {
    LINE_PATTERN
        .captures_iter(&str)
        .map(|c| c.index(1).to_string())
        .collect()
}

fn compose_multiline(header: &str, str: &String) -> String {
    let lines = lines_of(str);
    let mut delimiter = "";
    if lines.len() > 1 {
        delimiter = "\n";
    }
    format!("{}{}{}", header, delimiter, lines.join(delimiter))
}

pub fn meaning(definition: &String) -> String {
    compose_multiline("*Meaning*: ", definition)
}

pub fn as_in(example: &String) -> String {
    compose_multiline("*As in*: ", example)
}

#[cfg(test)]
mod tests {
    use crate::format::formatter::lines_of;
    use crate::format::meaning;

    #[test]
    fn parsing_multiline() {
        // GIVEN
        let text = r#"
1) "Stonewall" Jackson



2) Formerly a mafia-run gay bar the is famous for the riots that took place in 1969.

3) Short for the Stonewall riots, which occured in 1969 and helped to shape the modern GLBT rights movement."
"#.to_string();
        // WHEN
        let lines = lines_of(&text);

        // THEN
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn proper_multiline_meaning() {
        // GIVEN
        let text = r#"
1) "Stonewall" Jackson



2) Formerly a mafia-run gay bar the is famous for the riots that took place in 1969.

3) Short for the Stonewall riots, which occured in 1969 and helped to shape the modern GLBT rights movement."
"#.to_string();
        // WHEN
        let meaning = meaning(&text);

        // THEN
        assert!(meaning.starts_with("*Meaning*: \n"));
    }
    #[test]
    fn proper_single_line_meaning() {
        // GIVEN
        let text = "made or constructed by interlacing threads or strips of material or other elements into a whole".to_string();
        // WHEN
        let meaning = meaning(&text);
        // THEN
        let expected = format!("*Meaning*: {}", text);
        assert_eq!(expected, meaning);
    }
}
