///
/// This file is part of The Rust Raytracer.
///
/// The Rust Raytracer is free software: you can redistribute it
/// and/or modify it under the terms of the GNU General Public License
/// as published by the Free Software Foundation, either version 3 of
/// the License, or (at your option) any later version.
///
/// The Rust Raytracer is distributed in the hope that it will be
/// useful, but WITHOUT ANY WARRANTY; without even the implied
/// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
/// See the GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with The Rust Raytracer. If not, see
/// <https://www.gnu.org/licenses/>.
///

use rand::prelude::*;
use vec3::Vec3;
use ray::Ray;
use std::vec::Vec;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let mut vec: Vec3 = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen());

    loop {
        if vec.squared_length() >= 1.0 {
            return vec
        }

        vec.e[0] = rng.gen();
        vec.e[1] = rng.gen();
        vec.e[2] = rng.gen();
    }
}

pub struct Reflection {
    pub scattered: Ray,
    pub attenuation: Vec3,
    pub reflected: bool
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Reflection;

    fn albedo(&self) -> Vec3;
}


// Lambertian (diffuse) Material
pub struct Lambertian {
    albedo: Vec3,
}

// Metallic (reflective) Material
pub struct Metal {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &Hit) -> Reflection {
        let target: Vec3 = hit.p + hit.normal + random_in_unit_sphere();

        Reflection {
            scattered: Ray::new(hit.p, target - hit.p),
            attenuation: self.albedo(),
            reflected: true,
        }
    }

    fn albedo(&self) -> Vec3 {
        self.albedo
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Reflection {
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(r_in.direction()), &hit.normal);
        let scattered: Ray = Ray::new(hit.p, reflected);
        let direction: Vec3 = scattered.direction();

        Reflection {
            scattered: scattered,
            attenuation: self.albedo(),
            reflected: Vec3::dot(&direction, &hit.normal) > 0.0,
        }
    }

    fn albedo(&self) -> Vec3 {
        self.albedo
    }
}

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub object: &'a Hittable,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn material(&self) -> &Box<Material>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>,
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
    pub fn new(center: Vec3, radius: f32, material: Box<Material>) -> Sphere {
        Sphere { center, radius, material }
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
                return Some(Hit { t: tmp, p: p, normal: (p - self.center) / self.radius, object: self })
            }
        }

        None
    }

    fn material(&self) -> &Box<Material> {
        &self.material
    }
}

impl World {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
