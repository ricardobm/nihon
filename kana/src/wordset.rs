use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug, Clone, Default)]
pub struct WordSet {
    pub words: Vec<&'static Word>,
    pub chars: usize,
    pub missing: Vec<char>,
}

impl WordSet {
    /// Shuffles all `words` in the WordSet.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.words.as_mut_slice().shuffle(&mut rng);
    }

    /// Swaps the word at the given index with a random word at a
    /// higher index.
    pub fn swap_current(&mut self, index: usize) {
        use rand::Rng;

        let size = self.words.len();
        if index < size - 1 {
            let mut rng = thread_rng();
            let next_index: usize = rng.gen_range(index + 1, size);
            self.words.swap(index, next_index);
        }
    }
}

macro_rules! set_rare {
    () => {
        "ぺぢヌヅを"
    };
}

macro_rules! set_hiragana {
    () => ("あいうえおかきくけこがぎぐげごさしすせそざじずぜぞたちつてとだづでどなにぬねのはひふへほばびぶべぼぱぴぷぽまみむめもやゆよらりるれろわん")
}

macro_rules! set_katakana {
    () => ("アイウエオカキクケコガギグゲゴサシスセソザジズゼゾタチツテトダヂデドナニヌネノハヒフヘホバビブベボパピプペポマミムメモヤユヨラリルレロワヲン")
}

pub const SET_HIRAGANA: &'static str = set_hiragana!();
pub const SET_KATAKANA: &'static str = set_katakana!();
pub const SET_ALL: &'static str = concat!(set_hiragana!(), set_katakana!());
pub const SET_ALL_RARE: &'static str = concat!(set_hiragana!(), set_katakana!(), set_rare!());

use romaji::to_romaji;
use words::{Word, WORDS};

lazy_static! {
    pub static ref ALL_WORDS: Vec<&'static Word> = {
        let mut m = Vec::new();
        for it in WORDS.iter() {
            if is_valid_word(it) {
                m.push(it);
            }
        }
        m
    };
}

lazy_static! {
    pub static ref WORDS_BY_CHAR: HashMap<char, Vec<usize>> = {
        let mut m = HashMap::new();
        for (i, it) in ALL_WORDS.iter().enumerate() {
            for chr in it.word.chars() {
                let mut entry = m.entry(chr).or_insert(Vec::new());
                entry.push(i);
            }
        }
        m
    };
}

pub fn build_set(charset: &str, hint_len: usize) -> WordSet {
    // Build a set with all the required characters.
    let mut required = HashSet::new();
    for chr in charset.chars() {
        required.insert(chr);
    }

    // Indexes in ALL_WORDS for the words in the set we are building.
    let mut set_indexes: HashSet<usize> = HashSet::new();

    // Characters that are not found.
    let mut missing: HashSet<char> = HashSet::new();

    let mut chars = 0;

    let mut rng = thread_rng();

    fn choose_index(set: &HashSet<usize>, input: &Vec<usize>) -> Option<usize> {
        let mut rng = thread_rng();
        let mut indexes: Vec<usize> = Vec::new();
        for index in input {
            if !set.contains(index) {
                indexes.push(*index);
            }
        }
        if let Ok(value) = indexes.choose_weighted(&mut rng, |&idx| ALL_WORDS[idx].count) {
            Some(*value)
        } else {
            None
        }
    }

    // Add words to the set for each character in required.
    while required.len() > 0 && (hint_len == 0 || chars < hint_len) {
        // We choose one character at random to start so as to not
        // bias the resulting set.
        let vec: Vec<_> = required.iter().cloned().collect();
        let elem = vec.choose(&mut rng).unwrap();
        required.remove(elem);

        // Choose one of the words that contains the given character.
        let mut ok = false;
        if let Some(word_indexes) = WORDS_BY_CHAR.get(elem) {
            if let Some(index) = choose_index(&set_indexes, word_indexes) {
                // Add the word to the set.
                set_indexes.insert(index);
                ok = true;

                // Remove any character of this word from the required
                // set.
                for chr in ALL_WORDS[index].word.chars() {
                    chars += 1;
                    required.remove(&chr);
                }
            }
        }

        if !ok {
            missing.insert(*elem);
        }
    }

    for it in required {
        missing.insert(it);
    }

    // Completes the set
    let mut changed = true;
    let mut letters: Vec<_> = charset.chars().collect();
    'outer: while chars < hint_len && changed {
        {
            changed = false;
            letters.as_mut_slice().shuffle(&mut rng);
        }
        for chr in &letters {
            if let Some(word_indexes) = WORDS_BY_CHAR.get(&chr) {
                if let Some(index) = choose_index(&set_indexes, word_indexes) {
                    // Add the word to the set.
                    set_indexes.insert(index);
                    changed = true;

                    chars += ALL_WORDS[index].word.chars().count();
                    if chars >= hint_len {
                        break 'outer;
                    }
                }
            }
        }
    }

    let mut indexes: Vec<_> = set_indexes.iter().collect();
    indexes.sort();

    let mut words = Vec::new();
    for it in indexes {
        let row = ALL_WORDS[*it];
        words.push(row);
    }

    let mut missing: Vec<_> = missing.iter().cloned().collect();
    missing.sort();

    WordSet {
        words,
        chars,
        missing,
    }
}

fn is_valid_word(w: &Word) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[-a-zA-Z0-9]+$").unwrap();
    }
    let romaji = to_romaji(w.word);
    return RE.is_match(&romaji);
}
