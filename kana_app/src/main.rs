extern crate kana;
extern crate regex;
extern crate web_view;

#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_json;

extern crate rouille;

extern crate urlencoding;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[macro_use]
mod html;
mod server;

fn get_index() -> String {
    html!(
        "index.html",
        "es5-shim.js",
        "es6-shim.js",
        "base.js",
        "vue.js",
        "app.js",
        "styles.css"
    )
}

/// Application model.
struct Model {}

/// Messages that can be received from the JavaScript application.
#[derive(Serialize, Deserialize, Debug)]
enum Message {
    /// Reloads the web resources (on debug builds) and refreshes the
    /// web page.
    Refresh,

    /// Output an information message in the application console. This
    /// is called by a redirected `console.log` in the JS application.
    Console(String),

    /// Output an error message in the application console. This is
    /// called by a redirected `console.error` in the JS application.
    Error(String),
}

/// Messages that can be sent to the JavaScript application.
#[derive(Serialize, Deserialize, Debug)]
enum Command {
    /// Reloads the webview page.
    Refresh(bool),
}

fn main() {
    // let set = kana::build_set(kana::SET_ALL_RARE, 500);

    // println!(
    //     "\nLoaded {} words with {} chars\n",
    //     set.words.len(),
    //     set.chars
    // );

    // if set.missing.len() > 0 {
    //     let missing: Vec<_> = set.missing.iter().map(|x| x.to_string()).collect();
    //     println!("Missing: {}\n", missing.join(" "));
    // }

    // for it in set.words.iter() {
    //     println!("{} - {}", it.word, it.count);
    // }

    let content = get_index();
    let server = server::start();
    let url = format!("http://localhost:{}", server.port());

    // Set the initial content.
    server.set_content(&content);
    println!("\nInternal server started at {}\n", url,);

    let mut log_counter: u64 = 1;
    let model = &mut Model {};
    web_view::builder()
        .title("Kana")
        .content(web_view::Content::Url(url))
        .size(800, 600)
        .resizable(false)
        .user_data(model)
        .invoke_handler(|webview: &mut web_view::WebView<_>, arg| {
            let input: Result<Message> = serde_json::from_str(arg);
            match input {
                Ok(msg) => match msg {
                    Message::Refresh => {
                        // This will reload the content on debug
                        // builds.
                        let content = get_index();
                        server.set_content(&content);

                        // Tell the JS application to reload the page.
                        send_command(webview, Command::Refresh(true));
                    }

                    Message::Console(text) => {
                        for it in text.lines() {
                            println!("{:04}|LOG| {}", log_counter, it);
                        }
                        log_counter += 1;
                    }

                    Message::Error(text) => {
                        for it in text.lines() {
                            println!("{:04}|ERR| {}", log_counter, it);
                        }
                        log_counter += 1;
                    }
                },
                Err(err) => {
                    println!("Invalid message: {}\n{}\n\n", err, arg);
                }
            }
            Ok(())
        })
        .run()
        .unwrap();

    server.stop();
}

/// Send a command to the JavaScript application running in the web
/// view.
fn send_command<T>(webview: &mut web_view::WebView<T>, cmd: Command) {
    let data = serde_json::to_string(&cmd).unwrap();
    webview
        .eval(&format!("window.main.exec({})", data))
        .unwrap();
}
