use egor::math::Vec2;

#[derive(Clone, Debug, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[allow(dead_code)]
impl Rectangle {

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle { x, y, width, height }
    }

    pub fn is_inside(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn get_size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    pub fn get_center(&self) -> Vec2 {
        Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

}
