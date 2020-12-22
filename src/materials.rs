use crate::objects::HitData;
use crate::ray::Ray;
use rand::rngs::StdRng;
use rand::Rng;
use rand_distr::UnitSphere;
use ultraviolet::Vec3;

pub struct DiffuseMaterial {
    pub albedo: Vec3,
}

impl Material for DiffuseMaterial {
    fn scatter_ray(&self, _: &Ray, hit_data: &HitData, rng: &mut StdRng) -> RayScatterResult {
        let target = hit_data.point + hit_data.normal + Vec3::from(rng.sample(UnitSphere));
        RayScatterResult::Scattered {
            scattered_ray: Ray {
                origin: hit_data.point,
                direction: target - hit_data.point,
            },
            attenuation: self.albedo,
        }
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Self {
            albedo: self.albedo,
        })
    }
}

pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzziness: f32,
}

impl Material for MetalMaterial {
    fn scatter_ray(&self, ray: &Ray, hit_data: &HitData, rng: &mut StdRng) -> RayScatterResult {
        let v = ray.direction.normalized();
        let reflection_direction = v - 2.0 * v.dot(hit_data.normal) * hit_data.normal;
        let scattered_ray = Ray {
            origin: hit_data.point,
            direction: reflection_direction + self.fuzziness * Vec3::from(rng.sample(UnitSphere)),
        };
        if scattered_ray.direction.dot(hit_data.normal) > 0.0 {
            RayScatterResult::Scattered {
                scattered_ray,
                attenuation: self.albedo,
            }
        } else {
            RayScatterResult::Unscattered
        }
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Self {
            albedo: self.albedo,
            fuzziness: self.fuzziness,
        })
    }
}

pub trait Material {
    fn scatter_ray(&self, ray: &Ray, hit_data: &HitData, rng: &mut StdRng) -> RayScatterResult;
    fn clone(&self) -> Box<dyn Material>;
}

pub enum RayScatterResult {
    Unscattered,
    Scattered {
        scattered_ray: Ray,
        attenuation: Vec3,
    },
}
