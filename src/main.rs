#[macro_use]
extern crate rust_i18n;

mod engine;
mod game;

i18n!("languages");

fn main() -> Result<(), i32>{
    let res = crate::game::run();
    match res {
        0 => Ok(()),
        _ => Err(res),
    }
}
