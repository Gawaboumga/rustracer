use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct HitInfo {
    pub position: Vec3,
    pub normal: Vec3,
}

pub trait Geometry: Sync + Send {
    fn compute_hit(&self, ray: &Ray, hitinfo: Option<&mut HitInfo>) -> Option<f32>;

    fn get_color(&self, position: &Vec3) -> (f32, f32, f32);

    fn get_reflection_factor(&self) -> Option<f32>;

    fn get_transparency_factor(&self) -> Option<f32>;
}