use crate::objects::HitData;
use crate::ray::Ray;
use rand::rngs::SmallRng;
use rand::Rng;
use ultraviolet::Vec3;

pub enum Material {
    Diffuse { albedo: Vec3 },
    Metal { albedo: Vec3, fuzziness: f32 },
    Dielectric { index_of_refraction: f32 },
}

impl Material {
    pub fn scatter_ray(
        &self,
        ray: &Ray,
        hit_data: &HitData,
        rng: &mut SmallRng,
    ) -> RayScatterResult {
        match self {
            Material::Diffuse { albedo } => {
                let mut direction =
                    hit_data.normal + Vec3::from(rng.gen::<[f32; 3]>()).normalized();
                if direction.x < 1e-8 && direction.y < 1e-8 && direction.z < 1e-8 {
                    direction = hit_data.normal;
                }
                RayScatterResult::Scattered {
                    scattered_ray: Ray {
                        origin: hit_data.point,
                        direction,
                    },
                    attenuation: *albedo,
                }
            }

            Material::Metal { albedo, fuzziness } => {
                let v = ray.direction.normalized();
                let reflection_direction = v - 2.0 * v.dot(hit_data.normal) * hit_data.normal;
                let random_in_unit_sphere = loop {
                    let p = Vec3::from(rng.gen::<[f32; 3]>());
                    if p.mag_sq() < 1.0 {
                        break p;
                    }
                };
                let scattered_ray = Ray {
                    origin: hit_data.point,
                    direction: reflection_direction + *fuzziness * random_in_unit_sphere,
                };
                if scattered_ray.direction.dot(hit_data.normal) > 0.0 {
                    RayScatterResult::Scattered {
                        scattered_ray,
                        attenuation: *albedo,
                    }
                } else {
                    RayScatterResult::Unscattered
                }
            }

            Material::Dielectric {
                index_of_refraction,
            } => {
                todo!()
            }
        }
    }
}

pub enum RayScatterResult {
    Unscattered,
    Scattered {
        scattered_ray: Ray,
        attenuation: Vec3,
    },
}
