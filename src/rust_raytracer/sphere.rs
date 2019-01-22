use super::geometry::Vec3f;
use super::material::Material;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn build(center: Vec3f, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let l = self.center.clone().sub(orig);
        let tca = l.mul(dir);
        let d2 = l.mul(&l) - tca * tca;
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return None;
        }

        let thc = (radius2 - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }

        if t0 >= 0.0 {
            Some(t0)
        } else {
            None
        }
    }
}
