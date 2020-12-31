use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_add() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(0.0, 0.0);
        let v3 = Vec2::new(1.0, 1.0);

        assert_eq!(v1, v1 + v2);
        assert_eq!(Vec2::new(2.0, 3.0), v1 + v3);
        assert_eq!(v2, v2 + v2);
    }

    #[test]
    fn vec2_mul() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(0.0, 0.0);
        let v3 = Vec2::new(1.0, 1.0);

        assert_eq!(v1, 1.0 * v1);
        assert_eq!(v2, 1.0 * v2);
        assert_eq!(v3, 1.0 * v3);

        assert_eq!(v2, 0.0 * v1);
        assert_eq!(v2, 0.0 * v2);
        assert_eq!(v2, 0.0 * v3);
    }

    #[test]
    fn vec2_from() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::from((1.0, 2.0));
        let v3: Vec2 = (1.0, 2.0).into();

        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
    }

    #[test]
    fn vec3_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.0, 0.0, 0.0);
        let v3 = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(v1, v1 + v2);
        assert_eq!(Vec3::new(2.0, 3.0, 4.0), v1 + v3);
        assert_eq!(v2, v2 + v2);
    }

    #[test]
    fn vec3_mul() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(0.0, 0.0, 0.0);
        let v3 = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(v1, 1.0 * v1);
        assert_eq!(v2, 1.0 * v2);
        assert_eq!(v3, 1.0 * v3);

        assert_eq!(v2, 0.0 * v1);
        assert_eq!(v2, 0.0 * v2);
        assert_eq!(v2, 0.0 * v3);
    }

    #[test]
    fn vec3_from() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::from((1.0, 2.0, 3.0));
        let v3: Vec3 = (1.0, 2.0, 3.0).into();

        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
    }

    #[test]
    fn vec4_add() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let v3 = Vec4::new(1.0, 1.0, 1.0, 1.0);

        assert_eq!(v1, v1 + v2);
        assert_eq!(Vec4::new(2.0, 3.0, 4.0, 5.0), v1 + v3);
        assert_eq!(v2, v2 + v2);
    }

    #[test]
    fn vec4_mul() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let v3 = Vec4::new(1.0, 1.0, 1.0, 1.0);

        assert_eq!(v1, 1.0 * v1);
        assert_eq!(v2, 1.0 * v2);
        assert_eq!(v3, 1.0 * v3);

        assert_eq!(v2, 0.0 * v1);
        assert_eq!(v2, 0.0 * v2);
        assert_eq!(v2, 0.0 * v3);
    }

    #[test]
    fn vec4_from() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::from((1.0, 2.0, 3.0, 4.0));
        let v3: Vec4 = (1.0, 2.0, 3.0, 4.0).into();

        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
    }
}
