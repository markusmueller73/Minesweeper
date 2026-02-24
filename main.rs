#[macro_use]
extern crate rust_i18n;

mod engine;
mod game;

// This fallback is only for Windows versions
i18n!("languages", fallback = "en");

fn main() -> Result<(), i32>{
    // This works in Linux and MacOS (?)
    if let Ok(lang_env) = std::env::var("LANG") {
        // rust-i18n only accepts languages without the underscore and the UTF8 suffix
        let lang = &lang_env[0..=1];
        rust_i18n::set_locale(lang);
    }
    let res = crate::game::run();
    match res {
        0 => Ok(()),
        _ => Err(res),
    }
}
