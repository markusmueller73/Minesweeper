use std::collections::HashMap;
#[cfg(debug_assertions)]
use std::fs::{read, read_dir};
use egor::render::Graphics;

#[derive(Clone, Debug, Default)]
pub struct Assets {
    font: HashMap<String,Option<String>>,
    texture: HashMap<String,isize>
}

impl Assets {

    pub fn load_textures(&mut self, graphics: &mut Graphics) {
        if cfg!(debug_assertions) {

            self.texture.entry("logo".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/bomb.png")) as isize);
            self.texture.entry("button".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/button.png")) as isize);
            self.texture.entry("button_hover".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/button_hover.png")) as isize);
            self.texture.entry("digit_0".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_0.png")) as isize);
            self.texture.entry("digit_1".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_1.png")) as isize);
            self.texture.entry("digit_2".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_2.png")) as isize);
            self.texture.entry("digit_3".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_3.png")) as isize);
            self.texture.entry("digit_4".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_4.png")) as isize);
            self.texture.entry("digit_5".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_5.png")) as isize);
            self.texture.entry("digit_6".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_6.png")) as isize);
            self.texture.entry("digit_7".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_7.png")) as isize);
            self.texture.entry("digit_8".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_8.png")) as isize);
            self.texture.entry("digit_9".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/digit_9.png")) as isize);
            self.texture.entry("field".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/field.png")) as isize);
            self.texture.entry("field_revealed".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/field_revealed.png")) as isize);
            self.texture.entry("flag".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/flag.png")) as isize);
            self.texture.entry("kaboom".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/kaboom.png")) as isize);
            self.texture.entry("text_nb1".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb1.png")) as isize);
            self.texture.entry("text_nb2".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb2.png")) as isize);
            self.texture.entry("text_nb3".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb3.png")) as isize);
            self.texture.entry("text_nb4".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb4.png")) as isize);
            self.texture.entry("text_nb5".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb5.png")) as isize);
            self.texture.entry("text_nb6".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb6.png")) as isize);
            self.texture.entry("text_nb7".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb7.png")) as isize);
            self.texture.entry("text_nb8".to_ascii_lowercase())
                .insert_entry(graphics.load_texture(include_bytes!("../../assets/text_nb8.png")) as isize);
        } else {

            let path = match read_dir("./assets/") {
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
                        self.texture.entry(key).insert_entry(value as isize);
                    }
                }
            }
        }
    }

    pub fn load_fonts(&mut self, graphics: &mut Graphics) {
        self.font.entry("mines".to_ascii_lowercase())
            .insert_entry(graphics.load_font(include_bytes!("../../assets/mine-sweeper.otf")));
    }

    pub fn get_texture(&self, texture: &str) -> usize {
        *self.texture.get(texture).unwrap_or(&isize::MAX) as usize
    }

    pub fn get_font(&self, font: &str) -> String {
        self.font.get(font).unwrap().to_owned().unwrap_or_default()
    }

}
