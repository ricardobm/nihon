fn main() {
    for it in kana::WORDS.iter().take(20) {
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
