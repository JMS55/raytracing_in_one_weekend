use ultraviolet::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at_distance(&self, z: f32) -> Vec3 {
        self.direction.mul_add(Vec3::broadcast(z), self.origin)
    }
}
