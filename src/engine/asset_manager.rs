use std::collections::HashMap;
use egor::render::Graphics;

#[derive(Debug, Default)]
pub struct Assets {
    font: HashMap<String,String>,
    texture: HashMap<String,usize>
}

impl Assets {

    pub fn load_textures(&mut self, graphics: &mut Graphics) {
        self.texture.entry("bomb".to_ascii_lowercase())
            .insert_entry(graphics.load_texture(include_bytes!("../../assets/bomb.png")));
        self.texture.entry("field".to_ascii_lowercase())
            .insert_entry(graphics.load_texture(include_bytes!("../../assets/field.png")));
        self.texture.entry("field_revealed".to_ascii_lowercase())
            .insert_entry(graphics.load_texture(include_bytes!("../../assets/field_revealed.png")));
        self.texture.entry("flag".to_ascii_lowercase())
            .insert_entry(graphics.load_texture(include_bytes!("../../assets/flag.png")));
        self.texture.entry("kaboom".to_ascii_lowercase())
            .insert_entry(graphics.load_texture(include_bytes!("../../assets/kaboom.png")));
        self.texture.entry("qmark".to_ascii_lowercase())
            .insert_entry(graphics.load_texture(include_bytes!("../../assets/qmark.png")));
    }

    pub fn load_fonts(&mut self, graphics: &mut Graphics) {
        self.font.entry("mines".to_ascii_lowercase())
            .insert_entry(graphics.load_font(include_bytes!("../../assets/acknowledge.ttf")).unwrap());
        self.font.entry("digital".to_ascii_lowercase())
            .insert_entry(graphics.load_font(include_bytes!("../../assets/digital-dream.ttf")).unwrap());
    }

    pub fn get_texture(&self, texture: &str) -> usize {
        *self.texture.get(texture).unwrap_or(&usize::MAX)
    }

    pub fn get_font(&self, font: &str) -> String {
        self.font.get(font).unwrap_or(&String::from("")).to_owned()
    }

}
