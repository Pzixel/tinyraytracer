use super::geometry::Vec3f;

#[derive(Clone)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3f,
    pub specular_exponent: f32,
}

impl Material {
    pub fn build(
        refractive_index: f32,
        albedo: [f32; 4],
        diffuse_color: Vec3f,
        specular_exponent: f32,
    ) -> Self {
        Self {
            refractive_index,
            albedo,
            diffuse_color,
            specular_exponent,
        }
    }
}
