#[macro_use]
extern crate serde_derive;

mod gui;

mod wiki_api;
mod local_dec_tables;

fn main() {
    gui::main()
}
