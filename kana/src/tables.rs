use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// Enumeration for hiragana and katakana characters.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Kana {
    /// Either `ッ` or `っ`.
    SmallTsu(char),

    /// Small variants used in digraphs (e.g. `ゥ` and `ゅ`)
    Small(char, DigraphSuffix),

    /// Katakana long bar `ー`.
    Bar(char),

    /// A hiragana or katakana character that does not generate
    /// a custom digraph (i.e. the digraph is formed by the character
    /// own syllable).
    ///
    /// The first element is the character, the last is the romaji
    /// syllable.
    Chr(char, &'static str),

    /// A hiragana or katakana character that can also form a custom
    /// digraph.
    ///
    /// The first element is the character, second is the romaji
    /// syllable and last the additional digraph prefix.
    Dig(char, &'static str, DigraphPrefix),
}

impl Kana {
    /// Lookup a `Kana` character by its `char`.
    pub fn get(chr: char) -> Option<Kana> {
        TABLE_MAP.get(&chr).cloned()
    }

    /// Get the `char` for a `Kana` character.
    pub fn get_char(&self) -> char {
        match self {
            Kana::SmallTsu(chr) => *chr,
            Kana::Small(chr, _) => *chr,
            Kana::Bar(chr) => *chr,
            Kana::Chr(chr, _) => *chr,
            Kana::Dig(chr, _, _) => *chr,
        }
    }
}

// Digraph suffixes for the small variants of kana characters.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DigraphSuffix {
    Ya,
    Yu,
    Yo,
    A,
    I,
    U,
    E,
    O,
}

impl DigraphSuffix {
    pub fn as_str(&self) -> &'static str {
        match self {
            DigraphSuffix::Ya => "ya",
            DigraphSuffix::Yu => "yu",
            DigraphSuffix::Yo => "yo",
            DigraphSuffix::A => "a",
            DigraphSuffix::I => "i",
            DigraphSuffix::U => "u",
            DigraphSuffix::E => "e",
            DigraphSuffix::O => "o",
        }
    }
}

// Additional digraph prefixes for the kana characters.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DigraphPrefix {
    W, // ウ
    Y, // イ
    K, // ク
    G, // グ
}

impl DigraphPrefix {
    pub fn as_str(&self) -> &'static str {
        match self {
            DigraphPrefix::W => "wa",
            DigraphPrefix::Y => "ya",
            DigraphPrefix::K => "kwa",
            DigraphPrefix::G => "gwa",
        }
    }
}

static TABLE: &[Kana] = &[
    //
    // Katakana
    //
    Kana::SmallTsu('ッ'),
    Kana::Small('ャ', DigraphSuffix::Ya),
    Kana::Small('ュ', DigraphSuffix::Yu),
    Kana::Small('ョ', DigraphSuffix::Yo),
    Kana::Small('ァ', DigraphSuffix::A),
    Kana::Small('ィ', DigraphSuffix::I),
    Kana::Small('ゥ', DigraphSuffix::U),
    Kana::Small('ェ', DigraphSuffix::E),
    Kana::Small('ォ', DigraphSuffix::O),
    // Extra
    Kana::Chr('ヴ', "vu"),
    Kana::Bar('ー'),
    // A
    Kana::Chr('ア', "a"),
    Kana::Dig('イ', "i", DigraphPrefix::Y),
    Kana::Dig('ウ', "u", DigraphPrefix::W),
    Kana::Chr('エ', "e"),
    Kana::Chr('オ', "o"),
    // KA
    Kana::Chr('カ', "ka"),
    Kana::Chr('キ', "ki"),
    Kana::Dig('ク', "ku", DigraphPrefix::K),
    Kana::Chr('ケ', "ke"),
    Kana::Chr('コ', "ko"),
    // GA
    Kana::Chr('ガ', "ga"),
    Kana::Chr('ギ', "gi"),
    Kana::Dig('グ', "gu", DigraphPrefix::G),
    Kana::Chr('ゲ', "ge"),
    Kana::Chr('ゴ', "go"),
    // SA
    Kana::Chr('サ', "sa"),
    Kana::Chr('シ', "shi"),
    Kana::Chr('ス', "su"),
    Kana::Chr('セ', "se"),
    Kana::Chr('ソ', "so"),
    // ZA
    Kana::Chr('ザ', "za"),
    Kana::Chr('ジ', "ji"),
    Kana::Chr('ズ', "zu"),
    Kana::Chr('ゼ', "ze"),
    Kana::Chr('ゾ', "zo"),
    // TA
    Kana::Chr('タ', "ta"),
    Kana::Chr('チ', "chi"),
    Kana::Chr('ツ', "tsu"),
    Kana::Chr('テ', "te"),
    Kana::Chr('ト', "to"),
    // DA
    Kana::Chr('ダ', "da"),
    Kana::Chr('ヂ', "dji"),
    Kana::Chr('ヅ', "dzu"),
    Kana::Chr('デ', "de"),
    Kana::Chr('ド', "do"),
    // NA
    Kana::Chr('ナ', "na"),
    Kana::Chr('ニ', "ni"),
    Kana::Chr('ヌ', "nu"),
    Kana::Chr('ネ', "ne"),
    Kana::Chr('ノ', "no"),
    // HA
    Kana::Chr('ハ', "ha"),
    Kana::Chr('ヒ', "hi"),
    Kana::Chr('フ', "fu"),
    Kana::Chr('ヘ', "he"),
    Kana::Chr('ホ', "ho"),
    // BA
    Kana::Chr('バ', "ba"),
    Kana::Chr('ビ', "bi"),
    Kana::Chr('ブ', "bu"),
    Kana::Chr('ベ', "be"),
    Kana::Chr('ボ', "bo"),
    // PA
    Kana::Chr('パ', "pa"),
    Kana::Chr('ピ', "pi"),
    Kana::Chr('プ', "pu"),
    Kana::Chr('ペ', "pe"),
    Kana::Chr('ポ', "po"),
    // MA
    Kana::Chr('マ', "ma"),
    Kana::Chr('ミ', "mi"),
    Kana::Chr('ム', "mu"),
    Kana::Chr('メ', "me"),
    Kana::Chr('モ', "mo"),
    // YA
    Kana::Chr('ヤ', "ya"),
    Kana::Chr('ユ', "yu"),
    Kana::Chr('ヨ', "yo"),
    // RA
    Kana::Chr('ラ', "ra"),
    Kana::Chr('リ', "ri"),
    Kana::Chr('ル', "ru"),
    Kana::Chr('レ', "re"),
    Kana::Chr('ロ', "ro"),
    // WA
    Kana::Chr('ワ', "wa"),
    Kana::Chr('ヲ', "wo"),
    Kana::Chr('ン', "n"),
    //
    // Hiragana
    //
    Kana::SmallTsu('っ'),
    Kana::Small('ゃ', DigraphSuffix::Ya),
    Kana::Small('ゅ', DigraphSuffix::Yu),
    Kana::Small('ょ', DigraphSuffix::Yo),
    Kana::Small('ぁ', DigraphSuffix::A),
    Kana::Small('ぃ', DigraphSuffix::I),
    Kana::Small('ぅ', DigraphSuffix::U),
    Kana::Small('ぇ', DigraphSuffix::E),
    Kana::Small('ぉ', DigraphSuffix::O),
    // A
    Kana::Chr('あ', "a"),
    Kana::Chr('い', "i"),
    Kana::Chr('う', "u"),
    Kana::Chr('え', "e"),
    Kana::Chr('お', "o"),
    // KA
    Kana::Chr('か', "ka"),
    Kana::Chr('き', "ki"),
    Kana::Chr('く', "ku"),
    Kana::Chr('け', "ke"),
    Kana::Chr('こ', "ko"),
    // GA
    Kana::Chr('が', "ga"),
    Kana::Chr('ぎ', "gi"),
    Kana::Chr('ぐ', "gu"),
    Kana::Chr('げ', "ge"),
    Kana::Chr('ご', "go"),
    // SA
    Kana::Chr('さ', "sa"),
    Kana::Chr('し', "shi"),
    Kana::Chr('す', "su"),
    Kana::Chr('せ', "se"),
    Kana::Chr('そ', "so"),
    // ZA
    Kana::Chr('ざ', "za"),
    Kana::Chr('じ', "ji"),
    Kana::Chr('ず', "zu"),
    Kana::Chr('ぜ', "ze"),
    Kana::Chr('ぞ', "zo"),
    // TA
    Kana::Chr('た', "ta"),
    Kana::Chr('ち', "chi"),
    Kana::Chr('つ', "tsu"),
    Kana::Chr('て', "te"),
    Kana::Chr('と', "to"),
    // DA
    Kana::Chr('だ', "da"),
    Kana::Chr('ぢ', "dji"),
    Kana::Chr('づ', "dzu"),
    Kana::Chr('で', "de"),
    Kana::Chr('ど', "do"),
    // NA
    Kana::Chr('な', "na"),
    Kana::Chr('に', "ni"),
    Kana::Chr('ぬ', "nu"),
    Kana::Chr('ね', "ne"),
    Kana::Chr('の', "no"),
    // HA
    Kana::Chr('は', "ha"),
    Kana::Chr('ひ', "hi"),
    Kana::Chr('ふ', "fu"),
    Kana::Chr('へ', "he"),
    Kana::Chr('ほ', "ho"),
    // BA
    Kana::Chr('ば', "ba"),
    Kana::Chr('び', "bi"),
    Kana::Chr('ぶ', "bu"),
    Kana::Chr('べ', "be"),
    Kana::Chr('ぼ', "bo"),
    // PA
    Kana::Chr('ぱ', "pa"),
    Kana::Chr('ぴ', "pi"),
    Kana::Chr('ぷ', "pu"),
    Kana::Chr('ぺ', "pe"),
    Kana::Chr('ぽ', "po"),
    // MA
    Kana::Chr('ま', "ma"),
    Kana::Chr('み', "mi"),
    Kana::Chr('む', "mu"),
    Kana::Chr('め', "me"),
    Kana::Chr('も', "mo"),
    // YA
    Kana::Chr('や', "ya"),
    Kana::Chr('ゆ', "yu"),
    Kana::Chr('よ', "yo"),
    // RA
    Kana::Chr('ら', "ra"),
    Kana::Chr('り', "ri"),
    Kana::Chr('る', "ru"),
    Kana::Chr('れ', "re"),
    Kana::Chr('ろ', "ro"),
    // WA
    Kana::Chr('わ', "wa"),
    Kana::Chr('を', "wo"),
    Kana::Chr('ん', "n"),
];

lazy_static! {
    static ref TABLE_MAP: HashMap<char, Kana> = {
        let mut m: HashMap<char, Kana> = HashMap::new();
        for val in TABLE {
            let key = val.get_char();
            let mut entry = m.entry(key);
            if let Entry::Occupied(_) = entry {
                panic!("Character '{}' duplicated in TABLE", key);
            }
            entry.or_insert(*val);
        }
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_kana() {
        assert_eq!(Kana::get('ー').unwrap(), Kana::Bar('ー'));
        assert_eq!(Kana::get('ッ').unwrap(), Kana::SmallTsu('ッ'));
        assert_eq!(Kana::get('そ').unwrap(), Kana::Chr('そ', "so"));
        assert_eq!(Kana::get('x'), None);
    }
}
