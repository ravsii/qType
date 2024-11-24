use rand::seq::SliceRandom;
use std::io::{self};
use strum::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Language {
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
    words: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary { words: vec![] }
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

    pub fn random_words(&self, n: u32) -> Vec<String> {
        self.words
            .choose_multiple(&mut rand::thread_rng(), n as usize)
            .map(|s| s.to_owned())
            .collect()
    }
}
