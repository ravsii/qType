use rand::seq::SliceRandom;
use std::io::{self};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, Default)]
pub enum Language {
    #[default]
    English,
    Russian,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Russian",
        }
    }
}

#[derive(Debug)]
pub struct Dictionary {
    current: Language,
    words: Vec<String>,
}

impl Dictionary {
    pub fn new(lang: Language) -> io::Result<Self> {
        let mut d = Dictionary {
            current: lang,
            words: vec![],
        };

        d.load(lang)?;
        Ok(d)
    }

    /// load_dict_file accepts path for a text file, where words are
    /// separated by a newline (\n) char.
    pub fn load(&mut self, lang: Language) -> io::Result<()> {
        let dict_raw = match lang {
            Language::English => include_str!("../dict/en.dict"),
            Language::Russian => include_str!("../dict/ru.dict"),
        };

        self.words = dict_raw
            .lines()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        Ok(())
    }

    pub fn current(&self) -> Language {
        self.current
    }

    pub fn random_words(&self, n: usize) -> Vec<String> {
        self.words
            .choose_multiple(&mut rand::thread_rng(), n)
            .map(|s| s.to_owned())
            .collect()
    }
}
