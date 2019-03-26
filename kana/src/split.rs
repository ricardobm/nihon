use tables::*;

/// Representation for invalid `SmallTsu` characters.
const TSU: &'static str = "~tsu";

/// Invalid syllable. Must have two characters because of the digraph
/// logic.
const INVALID: &'static str = "~~";

// Split the kana text into romaji.
//
// This returns exactly one syllable per character in the original
// text, which means that digraphs and special characters cause
// what would be considered a syllable to be split.
pub fn split_romaji(text: &str) -> Vec<(String)> {
    let mut out: Vec<String> = Vec::new();
    let mut chars = text.chars();
    let mut tsu = (0, 0);

    // This appends `value` to `vec` while also taking care of
    // duplicating consonants after a `ッ` or `っ`.
    fn push_to(vec: &mut Vec<String>, value: String, tsu: (usize, usize)) -> (usize, usize) {
        let mut repeat = tsu.1;
        if repeat > 0 {
            let chr = if value == "chi" {
                't' // exceptionally, `chi` is doubled as `t-chi`
            } else {
                value.chars().next().unwrap()
            };
            while repeat > 0 {
                vec.push(chr.to_string());
                repeat -= 1;
            }
        }
        vec.push(value);
        (tsu.1, 0)
    }

    let mut last_prefix: &'static str = INVALID;
    while let Some(chr) = chars.next() {
        if let Some(chr) = Kana::get(chr) {
            match chr {
                // For normal characters we just add the romaji syllable.
                Kana::Chr(_, romaji) => {
                    tsu = push_to(&mut out, romaji.to_string(), tsu);
                    last_prefix = romaji;
                }

                // `Dig` is like a normal character, but changes when
                // used in a digraph.
                Kana::Dig(_, romaji, prefix) => {
                    tsu = push_to(&mut out, romaji.to_string(), tsu);
                    last_prefix = prefix.as_str();
                }

                // A `ッ` or `っ` causes the next consonant to be
                // doubled. We implement this by duplicating.
                Kana::SmallTsu(_) => {
                    tsu = (tsu.0, tsu.1 + 1);
                    last_prefix = INVALID;
                }

                // A long bar `ー` causes the previous vowel to be
                // doubled.
                Kana::Bar(chr) => {
                    let last = {
                        if let Some(last) = out.last() {
                            last.chars().last().unwrap_or(chr).to_string()
                        } else {
                            chr.to_string()
                        }
                    };
                    tsu = push_to(&mut out, last, tsu);
                }

                Kana::Small(_, suffix) => {
                    let cur_len = { out.len() };
                    if cur_len == 0 {
                        tsu = push_to(&mut out, suffix.as_str().to_string(), tsu);
                    } else {
                        // The suffix for this digraph.
                        let mut suffix = suffix.as_str();

                        // The prefix is the last syllable minus the vogal.
                        let prefix = &last_prefix[..last_prefix.len() - 1];

                        // `y-` suffixes drop the `y` with `chi`, `shi`, `ji`
                        // and `y` (e.g. `イョ`)
                        if suffix.starts_with("y")
                            && (prefix.ends_with("ch")
                                || prefix.ends_with("sh")
                                || prefix.ends_with("j")
                                || prefix.ends_with("y"))
                        {
                            suffix = &suffix[1..];
                        }

                        // Change the full syllable to the digraph prefix.
                        out[cur_len - 1] = String::from(prefix);

                        // Change any syllables added by a small TSU:
                        if tsu.0 > 0 {
                            let new_consonant = prefix.chars().next().unwrap();
                            while tsu.0 > 0 {
                                out[cur_len - 1 - tsu.0] = new_consonant.to_string();
                                tsu.0 -= 1;
                            }
                        }

                        // Append the suffix.
                        out.push(suffix.to_string());
                        last_prefix = INVALID;
                    }
                }
            }
        } else {
            tsu = push_to(&mut out, chr.to_string(), tsu);
        }
    }

    let mut tsu = tsu.1;
    while tsu > 0 {
        tsu -= 1;
        out.push(String::from(TSU));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_romaji_basic_cases() {
        // empty string
        assert_eq!(split_romaji(""), Vec::<String>::new());

        // non-kana passthrough
        assert_eq!(split_romaji("abc"), vec!("a", "b", "c"));

        // hiragana (basic)
        assert_eq!(
            split_romaji("あそび あそばせ"),
            vec!("a", "so", "bi", " ", "a", "so", "ba", "se")
        );

        // katakana (basic)
        assert_eq!(
            split_romaji("アソビ アソバセ"),
            vec!("a", "so", "bi", " ", "a", "so", "ba", "se")
        );

        // small tsu
        assert_eq!(split_romaji("だって"), vec!("da", "t", "te"));
        assert_eq!(split_romaji("ダッテ"), vec!("da", "t", "te"));

        // long bar
        assert_eq!(split_romaji("ハーハー"), vec!("ha", "a", "ha", "a"));

        // digraphs (basic)
        assert_eq!(
            split_romaji("きゃにゅびょ"),
            vec!("k", "ya", "n", "yu", "b", "yo")
        );
        assert_eq!(
            split_romaji("キャニュビョ"),
            vec!("k", "ya", "n", "yu", "b", "yo")
        );

        // digraphs (ch, sh, j)
        assert_eq!(
            split_romaji("しゃじゃちゃぢゃシェフュ"),
            vec!("sh", "a", "j", "a", "ch", "a", "dj", "a", "sh", "e", "f", "yu")
        );

        // digraph with replacement
        assert_eq!(split_romaji("イョ"), vec!("y", "o"));
        assert_eq!(split_romaji("ヴァ"), vec!("v", "a"));
        assert_eq!(split_romaji("ヴア"), vec!("vu", "a"));
        assert_eq!(split_romaji("ヴヴ"), vec!("vu", "vu"));
        assert_eq!(split_romaji("クォ"), vec!("kw", "o"));
    }

    #[test]
    fn test_tsu_with_digraph() {
        assert_eq!(split_romaji("ッイョ"), vec!("y", "y", "o"));
        assert_eq!(split_romaji("ッッイョ"), vec!("y", "y", "y", "o"));
        assert_eq!(split_romaji("ハッイョ"), vec!("ha", "y", "y", "o"));
        assert_eq!(
            split_romaji("ハッッイョ"),
            vec!("ha", "y", "y", "y", "o")
        );
        assert_eq!(
            split_romaji("ッハッッイョ"),
            vec!("h", "ha", "y", "y", "y", "o")
        );
        assert_eq!(
            split_romaji("ッハッッイイョ"),
            vec!("h", "ha", "i", "i", "i", "y", "o")
        );
        assert_eq!(
            split_romaji("ッハッッイッッッイョ"),
            vec!("h", "ha", "i", "i", "i", "y", "y", "y", "y", "o")
        );
    }
}
