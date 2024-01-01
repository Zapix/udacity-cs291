pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z}
    }

    pub fn position(&self) -> [f32; 3] {
        return [self.x, self.y, self.z]
    }
}