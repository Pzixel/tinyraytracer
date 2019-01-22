mod rust_raytracer;

use self::rust_raytracer::geometry::Vec3f;
use self::rust_raytracer::light::Light;
use self::rust_raytracer::material::Material;
use self::rust_raytracer::scene::Scene;
use self::rust_raytracer::sphere::Sphere;
use self::rust_raytracer::{render, save_ppm};

fn main() {
    let ivory = Material::build(1.0, [0.6, 0.3, 0.1, 0.0], Vec3f::build(0.4, 0.4, 0.3), 50.0);

    let glass = Material::build(
        1.5,
        [0.0, 0.5, 0.1, 0.8],
        Vec3f::build(0.6, 0.7, 0.8),
        125.0,
    );

    let red_rubber = Material::build(1.0, [0.9, 0.1, 0.0, 0.0], Vec3f::build(0.3, 0.1, 0.1), 10.0);

    let mirror = Material::build(
        1.0,
        [0.0, 10.0, 0.8, 0.0],
        Vec3f::build(1.0, 1.0, 1.0),
        1425.0,
    );

    let spheres: Vec<Sphere> = vec![
        Sphere::build(Vec3f::build(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::build(Vec3f::build(-1.0, -1.5, -12.0), 2.0, glass),
        Sphere::build(Vec3f::build(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::build(Vec3f::build(7.0, 5.0, -18.0), 4.0, mirror),
    ];

    let lights: Vec<Light> = vec![
        Light::build(Vec3f::build(-20.0, 20.0, 20.0), 1.5),
        Light::build(Vec3f::build(30.0, 50.0, -25.0), 1.8),
        Light::build(Vec3f::build(30.0, 20.0, 30.0), 1.7),
    ];

    let scene = Scene::build(spheres, lights);
    let framebuffer = render(&scene);
    save_ppm(&mut std::io::stdout(), &framebuffer).expect("Failed to save result file");
}
