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

pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod camera;

extern crate rand;

use rand::prelude::*;
use vec3::Vec3;
use ray::Ray;
use hittable::*;
use camera::Camera;

fn color(r: &Ray, world: &World, depth: i32) -> Vec3 {
    let hit: Option<Hit> = world.hit(r, 0.001, std::f32::MAX);

    match hit {
        Some(h) => {
            let reflection: Reflection = h.object.material().scatter(r, &h);

            if depth < 50 && reflection.reflected {
                reflection.attenuation * color(&reflection.scattered, world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        },
        None => {
            let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    const NX: i32 = 1600;
    const NY: i32 = 800;
    const NS: i32 = 100;

    let mut rng = thread_rng();

    let world: World = World {
        objects: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                                 0.5,
                                 Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))),
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0),
                                 0.3,
                                 Box::new(Metal::new(Vec3::new(0.9, 0.9, 0.9))))),
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                                 0.3,
                                 Box::new(Metal::new(Vec3::new(0.9, 0.9, 0.9))))),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                                 100.0,
                                 Box::new(Lambertian::new(Vec3::new(0.3, 0.3, 0.3))))),
        ],
    };

    let camera: Camera = Camera::default();

    // Print the PPM header
    println!("P3\n{} {}\n255", NX, NY);

    // Print the data
    for j in (0..=(NY-1)).rev() {
        for i in 0..NX {

            let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..NS {
                let ir: f32 = rng.gen();
                let jr: f32 = rng.gen();
                let u: f32 = (i as f32 + ir) / NX as f32;
                let v: f32 = (j as f32 + jr) / NY as f32;
                let r: Ray = camera.get_ray(u, v);
                col += color(&r, &world, 0);
            }

            col /= NS as f32;

            // Adjust gamma
            col.e[0] = col.e[0].sqrt();
            col.e[1] = col.e[1].sqrt();
            col.e[2] = col.e[2].sqrt();

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
