//mod vec3;
use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)] // TODO - do we need all of these
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new (origin: Point3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }
    fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
