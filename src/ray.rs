use vec3::Vec3;

#[allow(dead_code)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}


#[allow(dead_code)]
impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a, b}
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        return self.a + t * self.b
    }
}
