use std::fmt::Display;

#[derive(Clone)]
pub struct Entry {
    pub word: String,
    pub translation: String,
    pub part_of_speech: PartOfSpeech,
}

impl Entry {
    pub fn get_keywords(&self) -> Vec<String> {
        vec![self.word.to_string(), self.translation.to_string()]
    }
}

#[derive(Clone)]
pub enum PartOfSpeech {
    Noun,
    Verb,
}

impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Noun => write!(f, "n."),
            Self::Verb => write!(f, "v."),
        }
    }
}
