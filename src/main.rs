extern crate curl;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate kuchiki;
extern crate gtk;
extern crate gio;

mod gui;

mod wiki_api;

fn main() {
    gui::main()
}
