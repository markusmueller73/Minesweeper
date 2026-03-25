#[macro_use]
extern crate rust_i18n;

mod engine;
mod game;

i18n!("languages", fallback = "en");

fn main() -> Result<(), i32>{
    // This works only for Linux and MacOS
    if let Ok(env_lang) = std::env::var("LANG") {
        let lang = &env_lang[0..=1]; // Get the first two characters of the locale string
        rust_i18n::set_locale(lang);
    }
    crate::game::run()
}
