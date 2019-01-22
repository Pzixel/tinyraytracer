use super::light::Light;
use super::sphere::Sphere;

#[derive(Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn build(spheres: Vec<Sphere>, lights: Vec<Light>) -> Self {
        Self { spheres, lights }
    }
}
