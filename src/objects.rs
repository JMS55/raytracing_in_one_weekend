use crate::materials::Material;
use crate::ray::Ray;
use ultraviolet::Vec3;

pub struct ObjectList {
    pub objects: Vec<Object>,
}

impl ObjectList {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData> {
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

pub enum Object {
    Sphere {
        center: Vec3,
        radius: f32,
        material: Material,
    },
}

impl Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitData> {
        match self {
            Object::Sphere {
                center,
                radius,
                material,
            } => {
                let oc = ray.origin - *center;
                let a = ray.direction.dot(ray.direction);
                let b = oc.dot(ray.direction);
                let c = oc.dot(oc) - (radius * radius);
                let discriminant = (b * b) - (a * c);
                if discriminant > 0.0 {
                    let mut t = (-b - discriminant.sqrt()) / a;
                    if t < t_max && t > t_min {
                        let point = ray.at_distance(t);
                        return Some(HitData {
                            t,
                            point,
                            normal: (point - *center) / *radius,
                            material: &material,
                        });
                    }
                    t = (-b + discriminant.sqrt()) / a;
                    if t < t_max && t > t_min {
                        let point = ray.at_distance(t);
                        return Some(HitData {
                            t,
                            point,
                            normal: (point - *center) / *radius,
                            material: &material,
                        });
                    }
                }
                return None;
            }
        }
    }
}

pub struct HitData<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}
