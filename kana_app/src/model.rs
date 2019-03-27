use serde::{Deserialize, Serialize};

/// Sets of characters for training.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Set {
    /// Hiragana only.
    Hiragana,
    /// Katakana only.
    Katakana,
    /// Hiragana + Katakana
    All,
    /// Hiragana + Katakana + Rare
    Rare,
}

// Pages for the application.
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Page {
    Start,
    Training,
    Summary,
}

/// Application model.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    /// Current set selected.
    set: Set,

    /// Current page for the application.
    page: Page,

    /// Current word for the training set.
    word: String,

    /// Missing characters from the generated set.
    missing: Vec<char>,

    /// Number of hits for current training session.
    hits: usize,

    /// Number of misses for current training session.
    misses: usize,

    /// Number of words remaining in the training session.
    remaining: usize,

    /// Number of characters done for the training session.
    chars_done: usize,

    /// Total number of characters for the training session.
    chars_total: usize,

    /// Aggregated time from all the answers.
    answer_time: u64,

    /// Match for the last submitted kana.
    submitted: Option<kana::Match>,

    #[serde(skip)]
    word_set: kana::WordSet,

    #[serde(skip)]
    word_index: usize,
}

impl Model {
    /// Returns a new `Model`.
    pub fn new() -> Model {
        return Model {
            set: Set::Hiragana,
            page: Page::Start,

            word: String::new(),
            missing: Vec::new(),

            hits: 0,
            misses: 0,
            remaining: 0,
            chars_done: 0,
            chars_total: 0,
            answer_time: 0,
            submitted: None,

            word_set: Default::default(),
            word_index: 0,
        };
    }

    /// Starts a new training session.
    pub fn start(&mut self, set: Set, size: usize) {
        self.restart();
        self.set = set;
        self.page = Page::Training;

        let mut word_set = kana::build_set(
            match set {
                Set::Hiragana => kana::SET_HIRAGANA,
                Set::Katakana => kana::SET_KATAKANA,
                Set::All => kana::SET_ALL,
                Set::Rare => kana::SET_ALL_RARE,
            },
            size,
        );
        word_set.shuffle();

        self.missing = word_set.missing.clone();

        self.remaining = word_set.words.len();
        for it in word_set.words.iter() {
            self.chars_total += it.word.chars().count();
        }

        self.word_index = 0;
        self.word_set = word_set;
        self.word = String::from(self.word_set.words[0].word);
    }

    pub fn submit(&mut self, text: &str, elapsed_ms: u64) {
        let text = text.trim();
        self.submitted = None;
        if text.len() == 0 {
            return;
        }

        self.answer_time += elapsed_ms;

        let num_words = self.word_set.words.len();
        if self.word_index < num_words {
            let word = self.word_set.words[self.word_index];
            let s = kana::Match::new(word.word, text);
            if s.is_match {
                self.hits += 1;
                self.word_index += 1;
                self.remaining -= 1;
                self.chars_done += word.word.chars().count();
            } else {
                self.misses += 1;
                self.word_set.swap_current(self.word_index);
            }
            self.submitted = Some(s);

            if self.word_index < num_words {
                self.word = String::from(self.word_set.words[self.word_index].word);
            } else {
                self.page = Page::Summary;
            }
        }
    }

    /// Restarts the `Model` to the initial state.
    pub fn restart(&mut self) {
        self.page = Page::Start;
        self.word = String::new();
        self.missing = Vec::new();
        self.hits = 0;
        self.misses = 0;
        self.remaining = 0;
        self.chars_done = 0;
        self.chars_total = 0;
        self.answer_time = 0;
        self.submitted = None;
        self.word_set = Default::default();
        self.word_index = 0;
    }
}
