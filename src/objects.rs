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
                let a = ray.direction.mag_sq();
                let half_b = oc.dot(ray.direction);
                let c = oc.mag_sq() - (radius * radius);

                let discriminant = (half_b * half_b) - (a * c);
                if discriminant < 0.0 {
                    return None;
                }
                let sqrtd = discriminant.sqrt();

                let mut root = (-half_b - sqrtd) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + sqrtd) / a;
                    if root < t_min || t_max < root {
                        return None;
                    }
                }

                let point = ray.at_distance(root);
                let outward_normal = (point - *center) / *radius;
                let front_face = ray.direction.dot(outward_normal) < 0.0;
                let normal = if front_face {
                    outward_normal
                } else {
                    -outward_normal
                };
                Some(HitData {
                    t: root,
                    point,
                    normal,
                    front_face,
                    material,
                })
            }
        }
    }
}

pub struct HitData<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a Material,
}
