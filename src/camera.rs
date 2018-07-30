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

use ray::Ray;
use vec3::Vec3;
use std::f32::consts;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta: f32 = vfov * consts::PI / 180.0;
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect * half_height;

        let w: Vec3 = Vec3::unit_vector(&(lookfrom - lookat));
        let u: Vec3 = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v: Vec3 = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: lookfrom - half_width*u - half_height*v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom,
        }
    }

    pub fn default() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        );
    }
}
