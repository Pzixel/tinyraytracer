pub mod geometry;
pub mod light;
pub mod material;
pub mod scene;
pub mod sphere;

use std::sync::Arc;

use self::geometry::Vec3f;
use self::material::Material;
use self::scene::Scene;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const WF32: f32 = WIDTH as f32;
const HF32: f32 = HEIGHT as f32;
const FOV: f32 = std::f32::consts::PI / 2.0;
const THREADS: usize = 8;

type Framebuffer = Vec<Vec<Vec3f>>;

fn reflect(i: &Vec3f, n: &Vec3f) -> Vec3f {
    i.sub(&n.scale(2.0).scale(i.mul(n)))
}

fn refract(i: &Vec3f, n: &Vec3f, refractive_index: f32) -> Vec3f {
    let mut cosi = -(-1f32.max(1f32.min(i.mul(n))));
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let mut n_ = n.clone();

    if cosi < 0.0 {
        cosi = -cosi;
        std::mem::swap(&mut etai, &mut etat);
        n_ = n.reverse();
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3f::build(0.0, 0.0, 0.0)
    } else {
        i.scale(eta).add(&n_.scale(eta * cosi - k.sqrt()))
    }
}

fn scene_intersect(orig: &Vec3f, dir: &Vec3f, scene: &Scene) -> Option<(Vec3f, Vec3f, Material)> {
    let mut spheres_dist = std::f32::MAX;

    let initial = (
        Vec3f::build(0.0, 0.0, 0.0),
        Vec3f::build(0.0, 0.0, 0.0),
        scene.spheres[0].material.clone(),
    );

    let (mut hit, mut n, mut material) = scene.spheres.iter().fold(initial, |acc, sphere| {
        match sphere.ray_intersect(orig, dir) {
            None => acc,
            Some(dist_i) => {
                if dist_i < spheres_dist {
                    spheres_dist = dist_i;
                    let hit = orig.add(&dir.scale(dist_i));
                    let n = hit.sub(&sphere.center).normalize();
                    let material = sphere.material.clone();
                    (hit, n, material)
                } else {
                    acc
                }
            }
        }
    });

    let mut checkerboard_dist = std::f32::MAX;

    if dir.y.abs() > 1e-3 {
        let d = -(orig.y + 4.0) / dir.y;
        let pt = orig.add(&dir.scale(d));

        if d > 0.0 && pt.x.abs() < 10.0 && pt.z < -10.0 && pt.z > -30.0 && d < spheres_dist {
            checkerboard_dist = d;
            hit = pt;
            n = Vec3f::build(0.0, 1.0, 0.0);

            material.diffuse_color =
                if (0.5 * hit.x + 1000.0) as isize + (0.5 * hit.z) as isize & 1 == 1 {
                    Vec3f::build(1.0, 1.0, 1.0)
                } else {
                    Vec3f::build(1.0, 0.7, 0.3)
                };

            material.diffuse_color = material.diffuse_color.scale(0.3);
        }
    }

    if spheres_dist.min(checkerboard_dist) < 1000.0 {
        Some((hit, n, material))
    } else {
        None
    }
}

fn offset_orig(dir: &Vec3f, point: &Vec3f, n: &Vec3f) -> Vec3f {
    if dir.mul(n) < 0.0 {
        point.sub(&n.scale(1e-3))
    } else {
        point.add(&n.scale(1e-3))
    }
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, scene: &Scene, depth: usize) -> Vec3f {
    if let (true, Some((point, n, material))) = (depth <= 4, scene_intersect(orig, dir, scene)) {
        let reflect_dir = reflect(&dir, &n).normalize();
        let refract_dir = refract(&dir, &n, material.refractive_index).normalize();

        let reflect_orig = offset_orig(&reflect_dir, &point, &n);
        let refract_orig = offset_orig(&refract_dir, &point, &n);

        let reflect_color = cast_ray(&reflect_orig, &reflect_dir, scene, depth + 1);
        let refract_color = cast_ray(&refract_orig, &refract_dir, scene, depth + 1);

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;

        for light in scene.lights.iter() {
            let light_dir = light.position.sub(&point).normalize();
            let shadow_orig = offset_orig(&light_dir, &point, &n);
            let shadow_intersect = scene_intersect(&shadow_orig, &light_dir, scene);

            if let Some((shadow_point, _, _)) = shadow_intersect {
                let light_distance = light.position.sub(&point).norm();

                if shadow_point.sub(&shadow_orig).norm() < light_distance {
                    continue;
                }
            };

            diffuse_light_intensity += light.intensity * 0f32.max(light_dir.mul(&n));

            specular_light_intensity += 0f32
                .max(-reflect(&light_dir.reverse(), &n).mul(&dir))
                .powf(material.specular_exponent)
                * light.intensity;
        }

        material
            .diffuse_color
            .scale(diffuse_light_intensity * material.albedo[0])
            .add(&Vec3f::build(1.0, 1.0, 1.0).scale(specular_light_intensity * material.albedo[1]))
            .add(&reflect_color.scale(material.albedo[2]))
            .add(&refract_color.scale(material.albedo[3]))
    } else {
        Vec3f::build(0.2, 0.7, 0.8)
    }
}

pub fn render(scene: &Scene) -> Framebuffer {
    let scene_arc = Arc::new(scene.clone());
    let mut handles = vec![];

    for t in 0..THREADS {
        let scene = Arc::clone(&scene_arc);
        handles.push(std::thread::spawn(move || render_thread(t, &scene)));
    }

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn render_thread(index: usize, scene: &Scene) -> Vec<Vec3f> {
    let mut buffer = vec![];

    let rows_per_thread = (HF32 / THREADS as f32).ceil() as usize;
    let offset = index * rows_per_thread;
    let half_fov_tan = (FOV / 2.0).tan();

    for j in 0..rows_per_thread.min(HEIGHT - offset) {
        for i in 0..WIDTH {
            let x = (2.0 * (i as f32 + 0.5) / WF32 - 1.0) * half_fov_tan * WF32 / HF32;
            let y = -(2.0 * (offset as f32 + j as f32 + 0.5) / HF32 - 1.0) * half_fov_tan;
            let dir = Vec3f::build(x, y, -1.0).normalize();
            buffer.push(cast_ray(&Vec3f::build(0.0, 0.0, 0.0), &dir, scene, 0));
        }
    }

    buffer
}

pub fn save_ppm(out: &mut std::io::Write, framebuffer: &Framebuffer) -> Result<(), std::io::Error> {
    write!(out, "P6\n{} {}\n255\n", WIDTH, HEIGHT)?;

    for chunk in framebuffer.iter() {
        for vec3f in chunk.iter() {
            let pixel = vec3f.to_array();
            let mut buffer: [u8; 3] = [0; 3];

            for i in 0..3 {
                buffer[i] = (255.0 * 0f32.max(1f32.min(pixel[i]))) as u8;
            }

            out.write(&buffer)?;
        }
    }

    Ok(())
}
