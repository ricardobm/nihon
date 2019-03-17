use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let words = fs::read_to_string("words.txt").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("word_list.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"pub static WORD_LIST: &[(&'static str, i32)] = &[\n")
        .unwrap();

    for line in words.lines().skip(1) {
        if line.trim().len() == 0 {
            continue;
        }

        let mut iter = line.split(",").skip(1);
        let word = iter.next().unwrap();
        let occurrences = iter.next().unwrap().parse::<i32>().unwrap();
        write!(&mut f, "\t(\"{}\", {}),\n", word, occurrences).unwrap();
    }

    f.write_all(b"];\n\n").unwrap();
}
