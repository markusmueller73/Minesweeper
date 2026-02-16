use std::collections::HashMap;
use std::fs::{read, read_dir};
use egor::render::Graphics;

// pub fn load_resources(graphics: &mut Graphics, texture: &mut HashMap<&str,usize>) {
//     texture.entry("logo").insert_entry(graphics.load_texture(include_bytes!("../../resources/logo.png")));
// }

pub fn load_textures(graphics: &mut Graphics, texture: &mut HashMap<String,isize>) {

    let path = match read_dir("./resources/") {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    for entry in path {

        let file = match entry {
            Ok(file) => file.path(),
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let ext = file.extension().unwrap().to_string_lossy().into_owned();

        if ext.eq_ignore_ascii_case("png") {

            let data: Vec<u8> = read(&file).unwrap_or(Vec::new());

            if !data.is_empty() {
                let key = file.file_stem().unwrap().to_string_lossy().into_owned().to_ascii_lowercase();
                let value = graphics.load_texture(&data);
                texture.entry(key).insert_entry(value as isize);
            }

        }

    }

}
