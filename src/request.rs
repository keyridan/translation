use super::language::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    pub word: WordToTranslate,
    pub from_language: Language,
    pub to_language: Language,
}

type WordToTranslate = String;

impl Translation {
    pub fn new(word: &str) -> Translation {
        Translation {
            word: word.to_string(),
            from_language: Language::AUTO,
            to_language: Language::AUTO,
        }
    }

    pub fn from(mut self, from: Language) -> Translation {
        self.from_language = from;
        self
    }

    pub fn to(mut self, to: Language) -> Translation {
        self.to_language = to;
        self
    }
}