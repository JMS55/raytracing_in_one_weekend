mod materials;
mod objects;
mod ray;

use crate::ray::Ray;
use image::{ImageBuffer, RgbImage};
use materials::*;
use objects::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelExtend, ParallelIterator};
use ultraviolet::{Lerp, Vec3};

const IMAGE_WIDTH: u32 = 1280;
const IMAGE_HEIGHT: u32 = 640;
const SAMPLES_PER_PIXEL: u32 = 500;

fn main() {
    let camera = Camera {
        origin: Vec3::zero(),
        lower_left_corner: Vec3::new(-2.0, 1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, -2.0, 0.0),
    };

    let objects = ObjectList {
        objects: vec![
            Object::Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::DiffuseMaterial {
                    albedo: Vec3::new(0.8, 0.3, 0.3),
                },
            },
            Object::Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::DiffuseMaterial {
                    albedo: Vec3::new(0.8, 0.8, 0.0),
                },
            },
            Object::Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::MetalMaterial {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzziness: 0.3,
                },
            },
            Object::Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::MetalMaterial {
                    albedo: Vec3::new(0.8, 0.8, 0.8),
                    fuzziness: 1.0,
                },
            },
        ],
    };

    let mut image = Vec::with_capacity(IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize);
    image.par_extend(
        (0..(IMAGE_WIDTH * IMAGE_HEIGHT))
            .into_par_iter()
            .flat_map_iter(|i| {
                let x = i % IMAGE_WIDTH;
                let y = i / IMAGE_WIDTH;
                let mut rng = StdRng::seed_from_u64(x as u64 * y as u64);
                let mut pixel = Vec3::zero();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (x as f32 + rng.gen::<f32>()) / IMAGE_WIDTH as f32;
                    let v = (y as f32 + rng.gen::<f32>()) / IMAGE_HEIGHT as f32;
                    let ray = camera.raycast(u, v);
                    pixel += color(&ray, &objects, 0, &mut rng);
                }
                pixel /= SAMPLES_PER_PIXEL as f32;
                pixel = pixel.map(|c| c.sqrt()) * 255.99;
                vec![pixel.x as u8, pixel.y as u8, pixel.z as u8] // TODO: Replace with array when rust gets const generics
            }),
    );
    let image: RgbImage = ImageBuffer::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, image).unwrap();
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
