#[derive(Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn build(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        self.scale(1.0 / norm)
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn add(&self, other: &Vec3f) -> Vec3f {
        Vec3f::build(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3f) -> Vec3f {
        Vec3f::build(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn scale(&self, rhs: f32) -> Vec3f {
        Vec3f::build(self.x * rhs, self.y * rhs, self.z * rhs)
    }

    pub fn reverse(&self) -> Vec3f {
        Vec3f::build(-self.x, -self.y, -self.z)
    }
}
