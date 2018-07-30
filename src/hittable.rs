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
use std::i32;

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

// Dialectric Material
pub struct Dialectric {
    ref_idx: f32,
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

impl Dialectric {
    pub fn new(ref_idx: f32) -> Dialectric {
        Dialectric { ref_idx }
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
        let reflected: Vec3 = Vec3::reflect(&Vec3::unit_vector(&r_in.direction()), &hit.normal);
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

struct Refraction {
    refracted: Vec3
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Refraction> {
    let uv: Vec3 = Vec3::unit_vector(v);
    let dt: f32 = Vec3::dot(&uv, n);
    let discriminant: f32 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discriminant > 0.0 {
        Some(Refraction {
            refracted: ni_over_nt*(uv - dt*n) - discriminant.sqrt()*n
        })
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0: f32 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 *= r0;
    r0 + (1.0-r0) * (((1.0 - cosine) as i32).pow(5)) as f32
}


impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Reflection {
        let reflected: Vec3 = Vec3::reflect(&r_in.direction(), &hit.normal);
        let dot_positive: bool = Vec3::dot(&r_in.direction(), &hit.normal) > 0.0;

        let outward_normal: Vec3 = if dot_positive {
            -hit.normal
        } else {
            hit.normal
        };

        let ni_over_nt: f32 = if dot_positive {
            self.ref_idx
        } else {
            1.0 / self.ref_idx
        };

        let cosine: f32 = if dot_positive {
            self.ref_idx * Vec3::dot(&r_in.direction(), &hit.normal) / r_in.direction().length()
        } else {
            -Vec3::dot(&r_in.direction(), &hit.normal) / r_in.direction().length()
        };

        let refraction: Option<Refraction> = refract(&r_in.direction(), &outward_normal, ni_over_nt);

        let reflect_prob: f32 = match refraction {
            Some(_) => {
                schlick(cosine, self.ref_idx)
            },
            None => {
                1.0
            }
        };

        let refracted = match refraction {
            Some(r) => {
                r.refracted
            },
            None => {
                Vec3::new(0.0, 0.0, 0.0)
            }
        };

        let scattered: Ray = if random::<f32>() < reflect_prob {
            Ray::new(hit.p, reflected)
        } else {
            Ray::new(hit.p, refracted)
        };

        Reflection {
            scattered: scattered,
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            reflected: true,
        }
    }

    fn albedo(&self) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
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


///
/// A World is a collection of hittable objects, and the main
/// entry point for ray tracing.
///

impl World {
    pub fn new() -> World {
        World { objects: Vec::new() }
    }
}

impl World {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut hits: Vec<Hit> = Vec::new();
        let mut closest_so_far: f32 = t_max;

        for object in &self.objects {
            let hit: Option<Hit> = object.hit(r, t_min, closest_so_far);

            match hit {
                Some(h) => {
                    closest_so_far = h.t;
                    hits.push(h);
                },
                None => {}
            }
        }

        hits.pop()
    }
}
