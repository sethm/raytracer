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

use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f32;3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn unit_vector(v: Vec3) -> Vec3 {
        return v / v.length()
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Vec3::dot(v, n) * n
    }

    // Accessors for xyz / rgb
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            -(self.e[0] * v.e[2] - self.e[2] * v.e[0]),
            self.e[0] * v.e[1] - self.e[1] * v.e[0]
        )
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self)  {
        let k: f32 = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:.3} {:.3} {:.3}]", self.x(), self.y(), self.z())
    }
}

// So many operators

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl<'a> ops::Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(),
                  self.y() + rhs.y(),
                  self.z() + rhs.z())
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl<'a> ops::Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(),
                  self.y() - rhs.y(),
                  self.z() - rhs.z())
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() * rhs.x(),
                  self.y() * rhs.y(),
                  self.z() * rhs.z())
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x() / self.x(),
                  rhs.y() / self.y(),
                  rhs.z() / self.z())
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(t * self.x(),
                  t * self.y(),
                  t * self.z())
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t,
                  self.y() / t,
                  self.z() / t)
    }
}

impl<'a> ops::Div<f32> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t,
                  self.y() / t,
                  self.z() / t)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(),
                  self * v.y(),
                  self * v.z())
    }
}

impl<'a> ops::Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3::new(self * v.x(),
                  self * v.y(),
                  self * v.z())
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(v.x() / self,
                  v.y() / self,
                  v.z() / self)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn  add_assign(&mut self, v: Vec3) {
        self.e[0] += v.e[0];
        self.e[1] += v.e[1];
        self.e[2] += v.e[2];
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn  sub_assign(&mut self, v: Vec3) {
        self.e[0] -= v.e[0];
        self.e[1] -= v.e[1];
        self.e[2] -= v.e[2];
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, v: Vec3) {
        self.e[0] *= v.e[0];
        self.e[1] *= v.e[1];
        self.e[2] *= v.e[2];
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, v: Vec3) {
        self.e[0] /= v.e[0];
        self.e[1] /= v.e[1];
        self.e[2] /= v.e[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}
