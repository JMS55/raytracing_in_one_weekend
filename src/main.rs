mod materials;
mod objects;
mod ray;

use crate::ray::{Camera, Ray};
use image::{ImageBuffer, RgbImage};
use materials::{Material, RayScatterResult};
use objects::{Object, ObjectList};
use rand::rngs::SmallRng;
use rand::{thread_rng, Rng, SeedableRng};
use rayon::prelude::{IntoParallelIterator, ParallelExtend, ParallelIterator};
use ultraviolet::{Lerp, Vec3};

const IMAGE_WIDTH: u32 = 960;
const IMAGE_HEIGHT: u32 = 540;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: u32 = 50;

fn main() {
    let origin = Vec3::zero();
    let horizontal = Vec3::new((16.0 / 9.0) * 2.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::unit_z();
    let camera = Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
    };

    // let objects = generate_scene();
    let objects = ObjectList {
        objects: vec![
            Object::Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Diffuse {
                    albedo: Vec3::new(1.0, 0.5, 1.0),
                },
            },
            Object::Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Dielectric {
                    index_of_refraction: 1.5,
                },
            },
            Object::Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: -0.4,
                material: Material::Dielectric {
                    index_of_refraction: 1.5,
                },
            },
            Object::Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Diffuse {
                    albedo: Vec3::new(0.5, 0.5, 0.5),
                },
            },
            Object::Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzziness: 0.3,
                },
            },
        ],
    };

    raytrace(camera, objects);
}

fn generate_scene() -> ObjectList {
    let mut objects = Vec::new();
    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).mag() > 0.9 {
                let material_choice = rng.gen::<f32>();
                let material = if material_choice < 0.8 {
                    Material::Diffuse {
                        albedo: Vec3::new(rng.gen(), rng.gen(), rng.gen())
                            * Vec3::new(rng.gen(), rng.gen(), rng.gen()),
                    }
                } else if material_choice < 0.95 {
                    Material::Metal {
                        albedo: Vec3::new(
                            rng.gen_range(0.5..1.0),
                            rng.gen_range(0.5..1.0),
                            rng.gen_range(0.5..1.0),
                        ),
                        fuzziness: rng.gen_range(0.5..1.0),
                    }
                } else {
                    Material::Dielectric {
                        index_of_refraction: 1.5,
                    }
                };
                objects.push(Object::Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    ObjectList { objects }
}

fn raytrace(camera: Camera, objects: ObjectList) {
    let mut image = Vec::with_capacity(IMAGE_WIDTH as usize * IMAGE_HEIGHT as usize);
    image.par_extend(
        (0..(IMAGE_WIDTH * IMAGE_HEIGHT))
            .into_par_iter()
            .flat_map_iter(|i| {
                let x = i % IMAGE_WIDTH;
                let y = IMAGE_HEIGHT - i / IMAGE_WIDTH;
                let mut rng = SmallRng::seed_from_u64(i as u64);
                let mut pixel = Vec3::zero();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (x as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH as f32 - 1.0);
                    let v = (y as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT as f32 - 1.0);
                    let ray = camera.raycast(u, v);
                    pixel += color(&ray, &objects, 0, &mut rng);
                }
                pixel /= SAMPLES_PER_PIXEL as f32;
                pixel = pixel.map(|c| c.sqrt());
                pixel = pixel.clamped(Vec3::zero(), Vec3::broadcast(0.999)) * 256.0;
                // TODO: Replace with an array when arrays implement into_iter()
                use std::iter::once;
                once(pixel.x)
                    .chain(once(pixel.y))
                    .chain(once(pixel.z))
                    .map(|pixel| pixel as u8)
            }),
    );
    let image: RgbImage = ImageBuffer::from_raw(IMAGE_WIDTH, IMAGE_HEIGHT, image).unwrap();
    image.save("image.png").unwrap();
}

fn color(ray: &Ray, objects: &ObjectList, depth: u32, rng: &mut SmallRng) -> Vec3 {
    if let Some(hit_data) = objects.hit(ray, 0.001, f32::MAX) {
        if depth < MAX_DEPTH {
            match hit_data.material.scatter_ray(ray, &hit_data, rng) {
                RayScatterResult::Unscattered => Vec3::zero(),
                RayScatterResult::Scattered {
                    scattered_ray,
                    attenuation,
                } => attenuation * color(&scattered_ray, objects, depth + 1, rng),
            }
        } else {
            Vec3::zero()
        }
    } else {
        let t = 0.5 * (ray.direction.normalized().y + 1.0);
        Vec3::one().lerp(Vec3::new(0.5, 0.7, 1.0), t)
    }
}
