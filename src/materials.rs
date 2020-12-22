use crate::objects::HitData;
use crate::ray::Ray;
use rand::rngs::SmallRng;
use rand::Rng;
use rand_distr::UnitSphere;
use ultraviolet::Vec3;

pub enum Material {
    DiffuseMaterial { albedo: Vec3 },
    MetalMaterial { albedo: Vec3, fuzziness: f32 },
}

impl Material {
    pub fn scatter_ray(
        &self,
        ray: &Ray,
        hit_data: &HitData,
        rng: &mut SmallRng,
    ) -> RayScatterResult {
        match self {
            Material::DiffuseMaterial { albedo } => {
                let target = hit_data.point
                    + hit_data.normal
                    + Vec3::from(rng.gen::<[f32; 3]>()).normalized(); // Sample from surface of unit sphere
                RayScatterResult::Scattered {
                    scattered_ray: Ray {
                        origin: hit_data.point,
                        direction: target - hit_data.point,
                    },
                    attenuation: *albedo,
                }
            }
            Material::MetalMaterial { albedo, fuzziness } => {
                let v = ray.direction.normalized();
                let reflection_direction = v - 2.0 * v.dot(hit_data.normal) * hit_data.normal;
                let scattered_ray = Ray {
                    origin: hit_data.point,
                    direction: reflection_direction
                        + *fuzziness * Vec3::from(rng.sample(UnitSphere)),
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
