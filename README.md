# Kana practice app

A Japanese Kana training application to practice
the recognition of **hiragana** and **katakana** characters
by translating them to rōmaji using real words.

**NOTE** this is __Windows__ only, as it uses a webview for
the UI.

Main features (see [screenshots](#screenshots-and-features)
below):

* Can generate a random training set for *hiragana*, *katakana*
or both.
* Words are taken from a real word list. The choice of words is
random but it is weighted so common words have a higher chance
to appear.
* Can generate a training set including all chosen characters
(as long as the length is enough or if the option "All" is used).
* Error report highlighting the error and showing the correct
translation.
* Full report with statistics at the end of the training set.

This is a spare time project. The app was developed in Rust with
the purpose of learning the language while also creating a tool
that I missed while studying Japanese.

## Building

The project uses [Cargo](https://doc.rust-lang.org/cargo/),
so as long as you have Rust properly setup, you can build it
with `cargo build` and run with `cargo run`.

## Screenshots and features

Choose the training set (katakana, hiragana or both) and
length:

![Main menu](https://github.com/ricardobm/nihon/blob/master/docs/menu.png)

Note that length is in number of characters (not words) and
that choosing *"All"* generates the minimum set with all characters.

The guess screen displays the number of words, overall progress…

![Guess screen](https://github.com/ricardobm/nihon/blob/master/docs/guess.png)

…and the number of errors.

![Guess screen with error](https://github.com/ricardobm/nihon/blob/master/docs/guess2.png)

Mistakes are displayed in the next screen and highlighted:

![Showing a mistake](https://github.com/ricardobm/nihon/blob/master/docs/diff.png)

Pressing *Enter* dismisses the error popup and resets the timer
for the current screen.

At the end of the training set a report is shown with completion
time, overall status and time average per character (estimate):

![Report at the end](https://github.com/ricardobm/nihon/blob/master/docs/complete-1.png)

Mistakes are also broken down per character:

![Reporting mistakes](https://github.com/ricardobm/nihon/blob/master/docs/complete-2.png)

If the set length was not enough to include all characters, the
report screen will also display any missing ones, as shown above.
