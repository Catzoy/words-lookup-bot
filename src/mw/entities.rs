use crate::format::ToEscaped;

#[derive(Debug, Clone)]
pub struct FoundWords {
    pub common: Vec<String>,
    pub possible: Vec<String>,
}

impl ToEscaped for FoundWords {
    fn to_escaped(&self) -> Self {
        FoundWords {
            common: self.common.to_escaped(),
            possible: self.possible.to_escaped(),
        }
    }
}
