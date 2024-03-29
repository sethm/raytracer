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

extern crate rand;
extern crate sdl2;

pub mod vec3;
pub mod ray;
pub mod hittable;
pub mod camera;

use std::thread;

use std::time;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::prelude::*;
use vec3::Vec3;
use ray::Ray;
use hittable::*;
use camera::Camera;

use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::Arc;
use std::sync::mpsc::channel;

const NX: u32 = 640;
const NY: u32 = 480;
const NS: u32 = 100;
const NUM_THREADS: u32 = 6;

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
            let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
            let t: f32 = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

#[derive(Debug)]
struct RenderResult {
    offset: usize,
    data: Vec<u8>
}

fn render_line(line: u32, world: &Arc<World>, camera: &Arc<Camera>, pitch: usize) -> RenderResult {
    let mut data: Vec<u8> = Vec::new();
    let offset = (NY - 1 - line) as usize * pitch;
    let y = line as usize;
    let mut rng = thread_rng();

    for i in 0..NX {
        let x = i as usize;

        let mut col: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        for _ in 0..NS {
            let ir: f32 = rng.gen();
            let jr: f32 = rng.gen();
            let u: f32 = (x as f32 + ir) / NX as f32;
            let v: f32 = (y as f32 + jr) / NY as f32;

            let r: Ray = camera.get_ray(u, v);
            col += color(&r, &world, 0);
        }

        col /= NS as f32;

        // Adjust gamma
        col.e[0] = col.e[0].sqrt();
        col.e[1] = col.e[1].sqrt();
        col.e[2] = col.e[2].sqrt();

        let ir: u8 = (255.99 * col.r()) as u8;
        let ig: u8 = (255.99 * col.g()) as u8;
        let ib: u8 = (255.99 * col.b()) as u8;

        data.push(ir);
        data.push(ig);
        data.push(ib);
    }

    RenderResult {
        offset,
        data
    }
}

fn now() -> u64 {
    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    t.as_secs() * 1000 + t.subsec_nanos() as u64 / 1_000_000
}

fn main() {
    let start_time = now();
    let mut time_displayed = false;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Rust Raytracer", NX, NY)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
        PixelFormatEnum::RGB24, NX, NY).unwrap();

    let world: World = World {
        objects: vec![
            // Middle sphere
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),
                                 0.5,
                                 Box::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))),
            // Right sphere
            Box::new(Sphere::new(Vec3::new(1.5, 0.2, -1.5),
                                 0.7,
                                 Box::new(Metal::new(Vec3::new(0.6, 0.6, 0.9))))),

            // Left sphere
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0),
                                 0.5,
                                 Box::new(Dialectric::new(2.0)))),

            // Giant "ground" sphere
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0),
                                 100.0,
                                 Box::new(Lambertian::new(Vec3::new(0.3, 0.3, 0.3))))),
        ],
    };

    // let camera: Camera = Camera::default();
    let camera: Camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        50.0,
        NX as f32 / NY as f32
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut j = NY;
    let pitch = NX as usize * PixelFormatEnum::RGB24.byte_size_per_pixel();

    let shared_world = Arc::new(world);
    let shared_camera = Arc::new(camera);
    let (tx, rx) = channel();

    for thread_num in 0..NUM_THREADS {
        let sw = shared_world.clone();
        let sc = shared_camera.clone();
        let tx = tx.clone();
        let lines_per_block = NY / NUM_THREADS;
        let start_line = thread_num * lines_per_block;
        let end_line = (thread_num + 1) * lines_per_block;
        thread::spawn(move || {
            for line in start_line..end_line {
                let result = render_line(line, &sw, &sc, pitch);
                tx.send(result).unwrap();
            }
        });
    }

    'running: loop {
        if j > 0 {
            j -= 1;
            let result = rx.recv().unwrap();
            let slice = result.data.as_slice();

            texture.with_lock(None, |buffer: &mut [u8], _: usize| {
                for n in 0..slice.len() {
                    buffer[result.offset + n] = slice[n];
                }
            }).unwrap();

            canvas.copy(&texture, None, Some(Rect::new(0, 0, NX, NY))).unwrap();
            canvas.present();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        if j == 0 {
            if !time_displayed {
                println!("Rendering with {} threads took: {} ms", NUM_THREADS, now() - start_time);
                time_displayed = true;
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
