use std::collections::HashMap;

const LONG_MARK: char = 'ー';

const LONG_A: char = 'ā';
const LONG_I: char = 'ī';
const LONG_U: char = 'ū';
const LONG_E: char = 'ē';
const LONG_O: char = 'ō';

const LONG_N: &'static str = "n̄";

use tables::*;

lazy_static! {
    static ref KANA_MAP: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        for row in HIRAGANA {
            m.insert(row.0, row.1);
        }
        for row in KATAKANA {
            m.insert(row.0, row.1);
        }
        m
    };
}

pub fn is_match(kana: &str, s: &str) -> bool {
    let input = s.to_lowercase();
    let expected = to_romaji(kana);
    if input == expected {
        return true;
    }
    input == replace_long(&expected)
}

fn replace_long(input: &str) -> String {
    let mut out = String::from(input);
    out = out.replace("ā", "aa");
    out = out.replace("ī", "ii");
    out = out.replace("ū", "uu");
    out = out.replace("ē", "ee");
    out = out.replace("ō", "oo");
    out = out.replace("n̄", "nn");
    out
}

pub fn to_romaji(input: &str) -> String {
    let mut was_small_tsu = false;
    let mut out = String::new();

    let mut text = String::from(input);
    for row in DIGRAPH {
        text = text.replace(row.0, row.1);
    }

    for chr in text.chars() {
        if chr == LONG_MARK {
            let mut replace = '-';
            if out.ends_with("a") {
                replace = LONG_A;
            } else if out.ends_with("i") {
                replace = LONG_I;
            } else if out.ends_with("u") {
                replace = LONG_U;
            } else if out.ends_with("e") {
                replace = LONG_E;
            } else if out.ends_with("o") {
                replace = LONG_O;
            } else if out.ends_with("n") {
                replace = 'n';
            }

            if replace != '-' {
                let cur_len = out.len();
                out.truncate(cur_len - 1);
                if replace == 'n' {
                    out.push_str(LONG_N);
                } else {
                    out.push(replace);
                }
            } else {
                out.push(replace);
            }
            continue;
        }

        let romaji = kana_to_romaji(chr);

        // Handles a small TSU
        let is_small_tsu = romaji == SMALL_TSU_REPR;
        if is_small_tsu {
            if was_small_tsu {
                out.push_str(SMALL_TSU_REPR);
            }
            was_small_tsu = true;
            continue;
        }

        // If the last character was a small TSU, we want to
        // duplicate the consonant.
        if was_small_tsu {
            if romaji == "" {
                out.push_str(SMALL_TSU_REPR);
            } else if romaji == "chi" {
                out.push('t');
            } else {
                let next = romaji.chars().next().unwrap();
                match next {
                    // Only consonants should be duplicated
                    '~' | 'a' | 'i' | 'u' | 'e' | 'o' => out.push_str(SMALL_TSU_REPR),
                    chr => out.push(chr),
                }
            }
        }
        was_small_tsu = is_small_tsu;

        if romaji.starts_with("~y") && out.ends_with("i") {
            let cur_len = out.len();
            out.truncate(cur_len - 1);
            if out.ends_with("ch") || out.ends_with("sh") || out.ends_with("j") {
                out.push_str(&romaji[2..])
            } else {
                out.push_str(&romaji[1..])
            }
        } else if romaji != "" {
            out.push_str(romaji.as_str());
        } else {
            out.push(chr);
        }
    }

    if was_small_tsu {
        out.push_str(SMALL_TSU_REPR);
    }

    out
}

fn kana_to_romaji(chr: char) -> String {
    match KANA_MAP.get(&chr) {
        Some(&s) => String::from(s),
        None => {
            if chr >= 'a' && chr <= 'z' {
                chr.to_string()
            } else {
                String::new()
            }
        }
    }
}

// spell-checker: disable

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
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
            "-āīūēōn̄"
        ));
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
        assert_eq!(to_romaji("っっコ"), "~tsukko");
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
            "āīūēō shī shā n̄ x-"
        )
    }

    #[test]
    fn test_to_romaji_random_words() {
        assert_eq!(to_romaji("パーティー"), "pātī");
        assert_eq!(to_romaji("ディスク"), "disuku");
        assert_eq!(to_romaji("ファッション"), "fasshon");
        assert_eq!(to_romaji("フィクション"), "fikushon");
        assert_eq!(to_romaji("シェルター"), "sherutā");
        assert_eq!(to_romaji("ジェスチャー"), "jesuchā");
        assert_eq!(to_romaji("ハロウィーン"), "harowīn");
        assert_eq!(to_romaji("ソフトウェア"), "sofutowea");
        assert_eq!(to_romaji("フォーク"), "fōku");
        assert_eq!(to_romaji("フェア"), "fea");
        assert_eq!(to_romaji("チェス"), "chesu");
        assert_eq!(to_romaji("デュエット"), "dyuetto");
        assert_eq!(to_romaji("ストップウォッチ"), "sutoppuwotchi");
        assert_eq!(to_romaji("イェイ"), "yei");
        assert_eq!(to_romaji("タトゥ"), "tatu");
        assert_eq!(to_romaji("クォーツ"), "kwōtsu");
        assert_eq!(to_romaji("クォーク"), "kwōku");
        assert_eq!(to_romaji("モーツァルト"), "mōtsaruto");
        assert_eq!(to_romaji("プレッツェル"), "purettseru");
        assert_eq!(to_romaji("インテルメッツォ"), "interumettso");
        assert_eq!(to_romaji("フューチャー"), "fyūchā");
        assert_eq!(to_romaji("ヴァイオリン"), "vaiorin");
        assert_eq!(to_romaji("ヴィーナス"), "vīnasu");
        assert_eq!(to_romaji("ラヴ"), "ravu");
        assert_eq!(to_romaji("ベートーヴェン"), "bētōven");
        assert_eq!(to_romaji("ヴォーカリスト"), "vōkarisuto");
        assert_eq!(
            to_romaji("ドゥーイットユアセルフ"),
            "dūittoyuaserufu"
        );
        assert_eq!(to_romaji("エスクァイア"), "esukwaia");
        assert_eq!(to_romaji("クィントゥス"), "kwintusu");
        assert_eq!(to_romaji("クェンティン"), "kwentin");
        assert_eq!(to_romaji("グァンタナモ"), "gwantanamo");
        assert_eq!(to_romaji("ツィンメルマン"), "tsinmeruman");
        assert_eq!(to_romaji("テューリンゲン"), "tyūringen");
    }
}
