#[macro_use]
extern crate rust_i18n;

mod engine;
mod game;

i18n!("languages", fallback = "en");

fn main() {
    crate::game::run();
}
