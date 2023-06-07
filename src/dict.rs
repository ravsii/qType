use rand::seq::SliceRandom;
use std::cell::OnceCell;

static EN_WORDS: &str = include_str!("../dict/en.txt");

fn words() -> Vec<String> {
    let words: OnceCell<Vec<String>> = OnceCell::new();
    words
        .get_or_init(|| {
            EN_WORDS
                .lines()
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .to_owned()
}

pub fn random_word() -> String {
    words()
        .choose(&mut rand::thread_rng())
        .expect("random word")
        .to_string()
}
