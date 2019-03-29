use std::collections::HashMap;

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

    /// Set of words for the current training.
    #[serde(skip)]
    word_set: kana::WordSet,

    /// Current word in the training set.
    #[serde(skip)]
    word_index: usize,

    /// Errors by kana character.
    errors: HashMap<char, usize>,

    /// Average time spent per char.
    times: Vec<CharAverage>,

    /// Time spent for each word in the set.
    #[serde(skip)]
    word_time: Vec<u64>,
}

/// Report the average time spent for a kana char.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharAverage {
    kana: char,
    time: f64,
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

            errors: HashMap::new(),
            times: Vec::new(),
            word_time: Vec::new(),
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

        self.errors = HashMap::new();
        self.word_time = self.word_set.words.iter().map(|_x| 0).collect();
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

            self.word_time[self.word_index] += elapsed_ms;

            if s.is_match {
                self.hits += 1;
                self.word_index += 1;
                self.remaining -= 1;
                self.chars_done += word.word.chars().count();
            } else {
                self.misses += 1;

                // Compute the failed syllables
                for chr in &s.fails {
                    self.errors.entry(*chr).and_modify(|x| *x += 1).or_insert(1);
                }

                // Move the word to later in the set.
                let new_index = self.word_set.swap_current(self.word_index);
                self.word_time.swap(self.word_index, new_index);
            }

            self.submitted = Some(s);

            if self.word_index < num_words {
                self.word = String::from(self.word_set.words[self.word_index].word);
            } else {
                self.page = Page::Summary;
                self.compute_averages();
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

    fn compute_averages(&mut self) {
        // Calculate the total time for all words. We use that
        // to calculate the error.
        let total_time: f64 = self.word_time.iter().map(|&x| (x as f64) / 1000.0).sum();

        // Compute the simple average of all characters. This divides
        // the time for each word equally between its characters and
        // then averages the character times between words.

        let mut chr_cnt: HashMap<char, f64> = HashMap::new(); // Count for cumulative average
        let mut chr_avg: HashMap<char, f64> = HashMap::new(); // Char average
        for (i, t) in self.word_time.iter().enumerate() {
            let word_time = (*t as f64) / 1000.0;
            let word_str = self.word_set.words[i].word;
            let word_len = word_str.chars().count() as f64;
            let word_avg = word_time / word_len;
            println!("--> {} ({:.03}s)", word_str, word_time);
            for chr in word_str.chars() {
                let cnt = chr_cnt.entry(chr).or_default();
                chr_avg
                    .entry(chr)
                    .and_modify(|avg| {
                        // Compute the cumulative moving average
                        *avg = (word_avg + *avg * *cnt) / (*cnt + 1.0);
                    })
                    .or_insert(word_avg);
                *cnt += 1.0;
            }
        }

        let total_time_1 = total_time_from_average(&chr_cnt, &chr_avg);
        println!(
            "\n    AVG TOTAL TIME: {:.03}s ({:.03}s)\n\n{:?}\n",
            total_time_1, total_time, chr_avg,
        );

        // Compute an iterative average for the characters.
        //
        // In this case, instead of splitting the word time equally
        // between characters, we use the current character average
        // as a weight to calculate a new average.
        //
        // We repeat the iteration hoping to converge to the proper
        // values for the average of each character.
        //
        // We use the simple average as a starting point for the
        // calculation.
        let mut iter_avg = chr_avg.clone();

        // How much the new average value weights when recalculating
        // the average.
        const AVG_FACTOR: f64 = 0.7;

        for _ in 0..100 {
            for (i, &t) in self.word_time.iter().enumerate() {
                let word_time = (t as f64) / 1000.0;
                let word_str = self.word_set.words[i].word;
                let word_weight = word_str.chars().map(|ref chr| iter_avg[chr]).sum::<f64>();

                let mut chars: Vec<char> = word_str.chars().collect();
                chars.sort();
                let mut chars = chars.iter().peekable();
                while let Some(chr) = chars.next() {
                    // Skip repeated characters.
                    while let Some(&next_chr) = chars.peek() {
                        if chr == next_chr {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Current average time for the character.
                    let avg = iter_avg[chr];
                    // Weight of this char on the word
                    let weight = avg / word_weight;
                    // New average time.
                    let new_avg = word_time * weight;
                    iter_avg.insert(*chr, new_avg * AVG_FACTOR + avg * (1.0 - AVG_FACTOR));
                }
            }
        }

        let total_time_2 = total_time_from_average(&chr_cnt, &iter_avg);
        println!(
            "\n    AVG TOTAL TIME: {:.03}s ({:.03}s)\n\n{:?}\n",
            total_time_2, total_time, iter_avg,
        );

        let mut times = Vec::new();
        for (&chr, &time) in &iter_avg {
            times.push(CharAverage {
                kana: chr,
                time: time,
            });
        }

        times.sort_by(|a, b| {
            b.time
                .partial_cmp(&a.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        self.times = times;
    }
}

fn total_time_from_average(count: &HashMap<char, f64>, average: &HashMap<char, f64>) -> f64 {
    let mut total_time = 0.0;
    for (chr, &avg) in average.iter() {
        let cnt = count.get(chr).unwrap();
        total_time += avg * cnt;
    }
    return total_time;
}
