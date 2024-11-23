use rand::seq::SliceRandom;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct Dictionary {
    words: Vec<String>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary { words: vec![] }
    }

    /// load_dict_file accepts path for a text file, where words are
    /// separated by a newline (\n) char.
    pub fn load_dict_file(&mut self, path: &str) -> io::Result<()> {
        let file = File::open(path)?;

        let words = BufReader::new(file)
            .lines()
            .map(|e| match e {
                Ok(line) => line,
                Err(_) => todo!(),
            })
            .collect::<Vec<String>>();

        self.words = words;

        Ok(())
    }

    pub fn random_words(&self, n: u32) -> Vec<String> {
        self.words
            .choose_multiple(&mut rand::thread_rng(), n as usize)
            .map(|s| s.to_owned())
            .collect()
    }

    pub fn random_word(self) -> String {
        self.random_words(1).get(0).expect("random word").to_owned()
    }
}
