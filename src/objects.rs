use crate::materials::Material;
use crate::ray::Ray;
use ultraviolet::Vec3;

pub struct ObjectList {
    pub objects: Vec<Box<dyn Object>>,
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData> {
        let mut hit_data = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(object_hit_data) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = object_hit_data.t;
                hit_data = Some(object_hit_data);
            }
        }
        hit_data
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);
        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point = ray.at_distance(t);
                return Some(HitData {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
            t = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point = ray.at_distance(t);
                return Some(HitData {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }
        return None;
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData>;
}

pub struct HitData {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
}
