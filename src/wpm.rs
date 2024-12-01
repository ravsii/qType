use chrono::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct WpmCounter {
    is_started: bool,
    started_at: DateTime<Local>,

    chars_counters: u64,
    words_counter: u64,
}

pub struct CurrentWPS {
    pub chars_per_min: f64,
    pub words_per_min: f64,
}

impl WpmCounter {
    pub fn new() -> Self {
        Self {
            is_started: false,
            started_at: Local::now(),
            words_counter: 0,
            chars_counters: 0,
        }
    }

    pub fn is_started(&self) -> bool {
        self.is_started
    }

    pub fn start(&mut self) {
        if !self.is_started {
            self.started_at = Local::now();
            self.is_started = true;
        }
    }

    /// tick_word increases a word counter.
    fn tick_word(&mut self) {
        self.words_counter += 1;
    }

    /// tick_char increases a char counter.
    /// if c is a <space>, it also ticks a words counter.
    pub fn tick_char(&mut self, c: &char) {
        self.chars_counters += 1;
        if c == &' ' {
            self.tick_word();
        }
    }

    pub fn current_wpm(&self) -> CurrentWPS {
        let now = Local::now();
        let diff = now - self.started_at;

        let mut total_sec = diff.num_seconds() as u64;

        if total_sec == 0 {
            total_sec = 1;
        }

        // TODO: Better math, especially for wpm
        CurrentWPS {
            chars_per_min: self.chars_counters as f64 / total_sec as f64 * 60f64,
            words_per_min: (self.chars_counters / 5) as f64 / total_sec as f64 * 60f64,
        }
    }
}
