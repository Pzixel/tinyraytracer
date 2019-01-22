use super::geometry::Vec3f;

#[derive(Clone)]
pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}

impl Light {
    pub fn build(position: Vec3f, intensity: f32) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
