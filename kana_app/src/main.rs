extern crate kana;
extern crate regex;
extern crate web_view;

#[macro_use]
extern crate lazy_static;

#[macro_use]
mod html;

fn get_index() -> String {
    html!("index.html", "app.js", "styles.css")
}

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

    run_webview();
}

fn run_webview() {
    web_view::builder()
        .title("Kana")
        .content(web_view::Content::Html(get_index()))
        .size(800, 600)
        .resizable(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            if arg == "refresh" {
                let html = get_index();
                webview
                    .eval(&format!(
                        "reload('{}')",
                        html.replace("\\", "\\\\")
                            .replace("\n", "\\n")
                            .replace("'", "\\'")
                    ))
                    .unwrap();
            } else {
                println!("\nHandler invoked with: {}\n", arg);
                webview.eval(r#"showAlert('From Rust!');"#).unwrap();
            }
            Ok(())
        })
        .run()
        .unwrap();
}
