use crate::format::ToEscaped;
use crate::stands4::WordDefinition;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct WordleAnswer {
    pub(crate) solution: String,
    pub(crate) editor: String,
    pub(crate) days_since_launch: i32,
}

impl ToEscaped for WordleAnswer {
    /// Produces a new `WordleAnswer` with its string fields escaped for safe output.
    ///
    /// The returned value has `solution` and `editor` escaped; `days_since_launch` is copied unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// let wa = WordleAnswer {
    ///     solution: String::from("solution"),
    ///     editor: String::from("editor"),
    ///     days_since_launch: 42,
    /// };
    /// let escaped = wa.to_escaped();
    /// assert_eq!(escaped.days_since_launch, 42);
    /// ```
    fn to_escaped(&self) -> Self {
        Self {
            solution: self.solution.to_escaped(),
            editor: self.editor.to_escaped(),
            days_since_launch: self.days_since_launch,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WordleDayAnswer {
    pub(crate) day: String,
    pub(crate) answer: WordleAnswer,
    pub(crate) definitions: Vec<WordDefinition>,
}

impl ToEscaped for WordleDayAnswer {
    /// Create a new `WordleDayAnswer` with string content escaped in its nested fields.
    ///
    /// Escapes string data in the contained `answer` and each `WordDefinition` in `definitions`.
    /// The `day` field is copied unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    ///
    /// let original = WordleDayAnswer {
    ///     day: Local::now(),
    ///     answer: WordleAnswer {
    ///         solution: "<tag>".to_string(),
    ///         editor: "&editor".to_string(),
    ///         days_since_launch: 42,
    ///     },
    ///     definitions: Vec::new(),
    /// };
    ///
    /// let escaped = original.to_escaped();
    /// assert_ne!(escaped.answer.solution, original.answer.solution);
    /// assert_ne!(escaped.answer.editor, original.answer.editor);
    /// assert_eq!(escaped.day, original.day);
    /// ```
    fn to_escaped(&self) -> Self {
        Self {
            day: self.day.to_escaped(),
            answer: self.answer.to_escaped(),
            definitions: self.definitions.to_escaped(),
        }
    }
}
