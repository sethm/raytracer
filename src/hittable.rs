use vec3::Vec3;
use ray::Ray;
use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

pub struct World {
    pub objects: Vec<Box<Hittable>>,
}

impl World {
    pub fn new() -> World {
        World { objects: Vec::new() }
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc: Vec3 = r.origin() - self.center;
        let a: f32 = Vec3::dot(&r.direction(), &r.direction());
        let b: f32 = Vec3::dot(&oc, &r.direction());
        let c: f32 = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;

        if discriminant > 0.0 {
            let tmp: f32 = (-b - (b * b - a * c).sqrt()) / a;
            if tmp < t_max && tmp > t_min {
                let p: Vec3 = r.point_at_parameter(tmp);
                return Some(Hit { t: tmp, p: p, normal: (p - self.center) / self.radius })
            }
        }

        None
    }
}

impl Hittable for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let closest_so_far: f32 = t_max;

        for object in &self.objects {
            let hit: Option<Hit> = object.hit(r, t_min, closest_so_far);

            if hit.is_some() {
                return hit;
            }
        }

        None
    }
}
