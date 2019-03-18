extern crate kana;
extern crate web_view;

use web_view::*;

macro_rules! style {
    ( $x:expr ) => {
        format!(r#"<style type="text/css">{}</style>"#, include_str!($x))
    };
}

macro_rules! script {
    ( $x:expr ) => {
        format!(
            r#"<script type="text/javascript">{}</script>"#,
            include_str!($x)
        )
    };
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
    let html = format!(
        r#"
        <!doctype html>
        <html>
            <head>
                {styles}
            </head>
            <body>
                <!--[if lt IE 10]>
                <div>Please upgrade Internet Explorer to use this software.</div>
                <![endif]-->
                <!--[if gte IE 10 | !IE ]> <!-->
                {scripts}
                <![endif]-->
            </body>
        </html>
    "#,
        styles = style!("../js/styles.css"),
        scripts = script!("../js/app.js"),
    );

    web_view::builder()
        .title("Kana")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            println!("\nHandler invoked with: {}\n", arg);
            webview.eval(r#"showAlert('From Rust!');"#).unwrap();
            Ok(())
        })
        .run()
        .unwrap();
}
