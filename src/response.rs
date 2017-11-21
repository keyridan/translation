use super::language::Language;
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Translation {
    from: String,
    to: String,
    from_language: Language,
    translated_words: Vec<TranslatedWords>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranslatedWords {
    word: String,
    article: Option<String>,
}

impl Translation {
    pub fn new(from: &str, to: &str, from_language: Language) -> Translation {
        Translation {
            from: from.to_string(),
            to: to.to_string(),
            from_language,
            translated_words: Vec::new()
        }
    }

    pub fn with_words(mut self, words: Vec<TranslatedWords>) -> Translation {
        self.translated_words = words;
        self
    }

    pub fn parse(from: String, to: String, language: &str) -> Translation {
        Translation {
            from,
            to,
            from_language: Language::from_str(language).unwrap(),
            translated_words: Vec::new()
        }
    }
}

impl TranslatedWords {
    pub fn new(word: &str, article: Option<String>) -> TranslatedWords {
        TranslatedWords {
            word: word.to_string(),
            article
        }
    }
}