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
}

// extern crate azul;

// use azul::{
//     prelude::*,
//     widgets::{button::Button, label::Label},
// };

// struct CounterApp {
//     counter: usize,
// }

// impl Layout for CounterApp {
//     fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
//         let label = Label::new(format!("Counter: {}", self.counter)).dom();
//         let button = Button::with_label("Add")
//             .dom()
//             .with_callback(On::MouseUp, Callback(update_counter));
//         Dom::new(NodeType::Div).with_child(label).with_child(button)
//     }
// }

// fn main() {
//     let app = App::new(CounterApp { counter: 0 }, AppConfig::default());
//     let window = Window::new(WindowCreateOptions::default(), css::native()).unwrap();
//     app.run(window).unwrap();
// }

// fn update_counter(
//     app_state: &mut AppState<CounterApp>,
//     _info: &mut CallbackInfo<CounterApp>,
// ) -> UpdateScreen {
//     app_state.data.modify(|state| state.counter += 1);
//     Redraw
// }
