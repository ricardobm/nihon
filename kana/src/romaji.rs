use serde::{Deserialize, Serialize};

use diff;
use split::split_romaji;

pub fn to_romaji(input: &str) -> String {
    split_romaji(input).concat()
}

/// Result of matching a kana and romaji string.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    /// True if `kana` and `romaji` match.
    pub is_match: bool,

    /// The kana input string.
    pub kana: String,

    /// The romaji input string.
    pub romaji: String,

    /// The actual romaji translation for the given kana.
    pub actual: String,

    /// Split syllables of `kana`.
    pub split: Vec<char>,

    /// Diff between `romaji` and `kana`.
    pub diff: Vec<diff::Diff>,

    /// The failed kana chars.
    pub fails: Vec<char>,
}

impl Match {
    pub fn new(kana: &str, romaji: &str) -> Match {
        let syllables = split_romaji(kana);
        let romaji = romaji.to_lowercase().replace("-", "ー");
        let diff = diff::diff(&syllables, &romaji);
        let actual = syllables.concat();
        let is_match = diff.iter().all(|ref x| match x {
            diff::Diff::Same(_) => true,
            _ => false,
        });
        let split: Vec<_> = kana.chars().collect();
        let mut fails = Vec::new();

        let mut kana_index = 0;
        for it in &diff {
            match it {
                diff::Diff::Same(_) => {
                    kana_index += 1;
                }
                diff::Diff::Insert(_) | diff::Diff::Change(_, _) => {
                    fails.push(split[kana_index]);
                    kana_index += 1;
                }
                diff::Diff::Delete(_) => {}
            }
        }

        Match {
            is_match,
            kana: String::from(kana),
            romaji,
            actual,
            split,
            diff,
            fails,
        }
    }
}

// spell-checker: disable

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        fn is_match(kana: &str, romaji: &str) -> bool {
            Match::new(kana, romaji).is_match
        }

        assert!(is_match("", ""));
        assert!(is_match("abc", "abc"));
        assert!(is_match("abc", "ABC"));
        assert!(is_match("[あ]", "[a]"));
        assert!(is_match("あいうえお", "aiueo"));
        assert!(is_match("アイウエオ", "aiueo"));
        assert!(is_match("まって", "matte"));
        assert!(is_match("マッテ", "matte"));
        assert!(is_match(
            "ーアーイーウーエーオーンー",
            "-aaiiuueeoonn"
        ));
    }

    #[test]
    fn test_to_romaji_non_kana() {
        assert_eq!(to_romaji(""), "");
        assert_eq!(to_romaji("abc"), "abc");
        assert_eq!(to_romaji("[あ]"), "[a]");
    }

    #[test]
    fn test_to_romaji_hiragana() {
        assert_eq!(to_romaji("あいうえお"), "aiueo");
        assert_eq!(to_romaji("かきくけこ"), "kakikukeko");
        assert_eq!(to_romaji("がぎぐげご"), "gagigugego");
        assert_eq!(to_romaji("さしすせそ"), "sashisuseso");
        assert_eq!(to_romaji("ざじずぜぞ"), "zajizuzezo");
        assert_eq!(to_romaji("たちつてと"), "tachitsuteto");
        assert_eq!(to_romaji("だぢづでど"), "dadjidzudedo");
        assert_eq!(to_romaji("なにぬねの"), "naninuneno");
        assert_eq!(to_romaji("はひふへほ"), "hahifuheho");
        assert_eq!(to_romaji("ばびぶべぼ"), "babibubebo");
        assert_eq!(to_romaji("ぱぴぷぺぽ"), "papipupepo");
        assert_eq!(to_romaji("まみむめも"), "mamimumemo");
        assert_eq!(to_romaji("やゆよ"), "yayuyo");
        assert_eq!(to_romaji("らりるれろ"), "rarirurero");
        assert_eq!(to_romaji("わを"), "wawo");
        assert_eq!(to_romaji("ん"), "n");
    }

    #[test]
    fn test_to_romaji_katakana() {
        assert_eq!(to_romaji("アイウエオ"), "aiueo");
        assert_eq!(to_romaji("カキクケコ"), "kakikukeko");
        assert_eq!(to_romaji("ガギグゲゴ"), "gagigugego");
        assert_eq!(to_romaji("サシスセソ"), "sashisuseso");
        assert_eq!(to_romaji("ザジズゼゾ"), "zajizuzezo");
        assert_eq!(to_romaji("タチツテト"), "tachitsuteto");
        assert_eq!(to_romaji("ダヂヅデド"), "dadjidzudedo");
        assert_eq!(to_romaji("ナニヌネノ"), "naninuneno");
        assert_eq!(to_romaji("ハヒフヘホ"), "hahifuheho");
        assert_eq!(to_romaji("バビブベボ"), "babibubebo");
        assert_eq!(to_romaji("パピプペポ"), "papipupepo");
        assert_eq!(to_romaji("マミムメモ"), "mamimumemo");
        assert_eq!(to_romaji("ヤユヨ"), "yayuyo");
        assert_eq!(to_romaji("ラリルレロ"), "rarirurero");
        assert_eq!(to_romaji("ワヲ"), "wawo");
        assert_eq!(to_romaji("ン"), "n");
    }

    #[test]
    fn test_to_romaji_small_u() {
        assert_eq!(to_romaji("まって"), "matte");
        assert_eq!(to_romaji("マッテ"), "matte");
        assert_eq!(to_romaji("こっち"), "kotchi");
        assert_eq!(to_romaji("コッチ"), "kotchi");
        assert_eq!(to_romaji("あっさり"), "assari");
        assert_eq!(to_romaji("アッサリ"), "assari");
        assert_eq!(to_romaji("アッイェ"), "ayye");

        assert_eq!(to_romaji("あっ"), "a~tsu");
        assert_eq!(to_romaji("いっ"), "i~tsu");
        assert_eq!(to_romaji("うっ"), "u~tsu");
        assert_eq!(to_romaji("えっ"), "e~tsu");
        assert_eq!(to_romaji("おっ"), "o~tsu");
        assert_eq!(to_romaji("っっコ"), "kkko");
        assert_eq!(to_romaji("っ"), "~tsu");
        assert_eq!(to_romaji("ッ"), "~tsu");
    }

    #[test]
    fn test_to_romaji_y() {
        assert_eq!(
            to_romaji(
                "きゃ ぎゃ しゃ じゃ ちゃ にゃ ひゃ びゃ ぴゃ みゃ りゃ"
            ),
            "kya gya sha ja cha nya hya bya pya mya rya"
        );
        assert_eq!(
            to_romaji(
                "きゅ ぎゅ しゅ じゅ ちゅ にゅ ひゅ びゅ ぴゅ みゅ りゅ"
            ),
            "kyu gyu shu ju chu nyu hyu byu pyu myu ryu"
        );
        assert_eq!(
            to_romaji(
                "きょ ぎょ しょ じょ ちょ にょ ひょ びょ ぴょ みょ りょ"
            ),
            "kyo gyo sho jo cho nyo hyo byo pyo myo ryo"
        );

        assert_eq!(
            to_romaji(
                "キャ ギャ シャ ジャ チャ ニャ ヒャ ビャ ピャ ミャ リャ"
            ),
            "kya gya sha ja cha nya hya bya pya mya rya"
        );
        assert_eq!(
            to_romaji(
                "キュ ギュ シュ ジュ チュ ニュ ヒュ ビュ ピュ ミュ リュ"
            ),
            "kyu gyu shu ju chu nyu hyu byu pyu myu ryu"
        );
        assert_eq!(
            to_romaji(
                "キョ ギョ ショ ジョ チョ ニョ ヒョ ビョ ピョ ミョ リョ"
            ),
            "kyo gyo sho jo cho nyo hyo byo pyo myo ryo"
        );
    }

    #[test]
    fn test_to_romaji_digraph() {
        assert_eq!(to_romaji("シャシシュシェショ"), "shashishushesho");
        assert_eq!(to_romaji("ジャジジュジェジョ"), "jajijujejo");
        assert_eq!(to_romaji("タティトゥテト"), "tatituteto");
        assert_eq!(to_romaji("ダディドゥデド"), "dadidudedo");
        assert_eq!(to_romaji("チャチチュチェチョ"), "chachichuchecho");
        assert_eq!(to_romaji("ファフィフフェフォ"), "fafifufefo");
        assert_eq!(to_romaji("ワウィウウェウォ"), "wawiuwewo");
        assert_eq!(to_romaji("ヴァヴィヴヴェヴォ"), "vavivuvevo");
    }

    #[test]
    fn test_to_romaji_long() {
        assert_eq!(
            to_romaji("アーイーウーエーオー シー シャー ンー xー"),
            "aaiiuueeoo shii shaa nn xx"
        )
    }

    #[test]
    fn test_to_romaji_random_words() {
        assert_eq!(to_romaji("パーティー"), "paatii");
        assert_eq!(to_romaji("ディスク"), "disuku");
        assert_eq!(to_romaji("ファッション"), "fasshon");
        assert_eq!(to_romaji("フィクション"), "fikushon");
        assert_eq!(to_romaji("シェルター"), "sherutaa");
        assert_eq!(to_romaji("ジェスチャー"), "jesuchaa");
        assert_eq!(to_romaji("ハロウィーン"), "harowiin");
        assert_eq!(to_romaji("ソフトウェア"), "sofutowea");
        assert_eq!(to_romaji("フォーク"), "fooku");
        assert_eq!(to_romaji("フェア"), "fea");
        assert_eq!(to_romaji("チェス"), "chesu");
        assert_eq!(to_romaji("デュエット"), "dyuetto");
        assert_eq!(to_romaji("ストップウォッチ"), "sutoppuwotchi");
        assert_eq!(to_romaji("イェイ"), "yei");
        assert_eq!(to_romaji("タトゥ"), "tatu");
        assert_eq!(to_romaji("クォーツ"), "kwootsu");
        assert_eq!(to_romaji("クォーク"), "kwooku");
        assert_eq!(to_romaji("モーツァルト"), "mootsaruto");
        assert_eq!(to_romaji("プレッツェル"), "purettseru");
        assert_eq!(to_romaji("インテルメッツォ"), "interumettso");
        assert_eq!(to_romaji("フューチャー"), "fyuuchaa");
        assert_eq!(to_romaji("ヴァイオリン"), "vaiorin");
        assert_eq!(to_romaji("ヴィーナス"), "viinasu");
        assert_eq!(to_romaji("ラヴ"), "ravu");
        assert_eq!(to_romaji("ベートーヴェン"), "beetooven");
        assert_eq!(to_romaji("ヴォーカリスト"), "vookarisuto");
        assert_eq!(
            to_romaji("ドゥーイットユアセルフ"),
            "duuittoyuaserufu"
        );
        assert_eq!(to_romaji("エスクァイア"), "esukwaia");
        assert_eq!(to_romaji("クィントゥス"), "kwintusu");
        assert_eq!(to_romaji("クェンティン"), "kwentin");
        assert_eq!(to_romaji("グァンタナモ"), "gwantanamo");
        assert_eq!(to_romaji("ツィンメルマン"), "tsinmeruman");
        assert_eq!(to_romaji("テューリンゲン"), "tyuuringen");
    }
}
