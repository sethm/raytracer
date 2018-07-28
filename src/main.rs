pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod camera;

extern crate rand;

use rand::prelude::*;
use vec3::Vec3;
use ray::Ray;
use hittable::{Hittable, Hit, Sphere, World};
use camera::Camera;

fn color(r: &Ray, world: &World) -> Vec3 {
    let hit: Option<Hit> = world.hit(r, 0.0, std::f32::MAX);

    match hit {
        Some(h) => {
            0.5 * Vec3::new(h.normal.x() + 1.0, h.normal.y() + 1.0, h.normal.z() + 1.0)
        },
        None => {
            let unit_direction: Vec3 = Vec3::unit_vector(r.direction());
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    const NX: i32 = 400;
    const NY: i32 = 200;
    const NS: i32 = 100;

    let mut rng = rand::thread_rng();

    let world: World = World {
        objects: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
        ]
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
                col += color(&r, &world);
            }

            col /= NS as f32;

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
