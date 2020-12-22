mod materials;
mod objects;
mod ray;

use crate::ray::Ray;
use image::{ImageBuffer, Rgb, RgbImage};
use materials::*;
use objects::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use ultraviolet::{Lerp, Vec3};

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 640;
const SAMPLES_PER_PIXEL: u32 = 500;

fn main() {
    let mut image: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = StdRng::seed_from_u64(0);

    let camera = Camera {
        origin: Vec3::zero(),
        lower_left_corner: Vec3::new(-2.0, 1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, -2.0, 0.0),
    };

    let objects = ObjectList {
        objects: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Box::new(DiffuseMaterial {
                    albedo: Vec3::new(0.8, 0.3, 0.3),
                }),
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Box::new(DiffuseMaterial {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                }),
            }),
            Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Box::new(MetalMaterial {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzziness: 0.3,
                }),
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Box::new(MetalMaterial {
                    albedo: Vec3::new(0.8, 0.8, 0.8),
                    fuzziness: 1.0,
                }),
            }),
        ],
    };

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let mut pixel = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + rng.gen::<f32>()) / IMAGE_WIDTH as f32;
                let v = (y as f32 + rng.gen::<f32>()) / IMAGE_HEIGHT as f32;
                let ray = camera.raycast(u, v);
                pixel += color(&ray, &objects, 0, &mut rng);
            }
            pixel /= SAMPLES_PER_PIXEL as f32;
            pixel = pixel.map(|c| c.sqrt()) * 255.99;
            image.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    image.save("image.png").unwrap();
}

fn color(ray: &Ray, objects: &ObjectList, depth: u32, rng: &mut StdRng) -> Vec3 {
    if let Some(hit_data) = objects.hit(ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            match hit_data.material.scatter_ray(ray, &hit_data, rng) {
                RayScatterResult::Unscattered => Vec3::new(0.0, 0.0, 0.0),
                RayScatterResult::Scattered {
                    scattered_ray,
                    attenuation,
                } => attenuation * color(&scattered_ray, objects, depth + 1, rng),
            }
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let t = 0.5 * (ray.direction.normalized().y + 1.0);
        Vec3::one().lerp(Vec3::new(0.5, 0.7, 1.0), t)
    }
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn raycast(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (u * self.horizontal) + (v * self.vertical),
        }
    }
}
