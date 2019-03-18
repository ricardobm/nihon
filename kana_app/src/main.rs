extern crate kana;
extern crate web_view;

use web_view::*;

fn main() {
    let set = kana::build_set(kana::SET_ALL_RARE, 500);

    println!(
        "\nLoaded {} words with {} chars\n",
        set.words.len(),
        set.chars
    );

    if set.missing.len() > 0 {
        let missing: Vec<_> = set.missing.iter().map(|x| x.to_string()).collect();
        println!("Missing: {}\n", missing.join(" "));
    }

    for it in set.words.iter() {
        println!("{} - {}", it.word, it.count);
    }

    web_view::builder()
        .title("Minimal webview example")
        .content(Content::Url("https://en.m.wikipedia.org/wiki/Main_Page"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
