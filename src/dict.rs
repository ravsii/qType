use rand::seq::SliceRandom;
use std::cell::OnceCell;

static EN_WORDS: &str = include_str!("../dict/en.txt");

fn words() -> Vec<String> {
    let words: OnceCell<Vec<String>> = OnceCell::new();
    words
        .get_or_init(|| {
            println!("init");
            EN_WORDS
                .lines()
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .to_owned()
}

pub fn random_words(n: usize) -> Vec<String> {
    words()
        .choose_multiple(&mut rand::thread_rng(), n)
        .map(|s| s.to_owned())
        .collect()
}

pub fn random_word() -> String {
    words()
        .choose(&mut rand::thread_rng())
        .expect("random word")
        .to_string()
}
