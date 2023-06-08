use std::{ops::{Add, Div, Mul, Neg, Sub}, simd::f32x4};

use rand_chacha::ChaCha20Rng;

use crate::rand_double::{rand_double, rand_double_range};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Self) -> Self {
        //Self {
        //    x: self.x * _rhs.x,
        //    y: self.y * _rhs.y,
        //    z: self.z * _rhs.z,
        //}
        return Vec3::from_simd4(
            self.to_simd4() * _rhs.to_simd4()
        );
    }
}
impl Mul<i64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: i64) -> Self {
        //Self {
        //    x: self.x * _rhs as f32,
        //    y: self.y * _rhs as f32,
        //    z: self.z * _rhs as f32,
        //}
        return Vec3::from_simd4(
            self.to_simd4() * f32x4::from_array([_rhs as f32, _rhs as f32, _rhs as f32, 1.0])
        );
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: f32) -> Self {
        //Self {
        //    x: self.x * _rhs,
        //    y: self.y * _rhs,
        //    z: self.z * _rhs,
        //}
        return Vec3::from_simd4(
            self.to_simd4() * f32x4::from_array([_rhs, _rhs, _rhs, 1.0])
        );
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Self) -> Self {
        //Self {
        //    x: self.x / _rhs.x,
        //    y: self.y / _rhs.y,
        //    z: self.z / _rhs.z,
        //}
        return Vec3::from_simd4(
            self.to_simd4() / _rhs.to_simd4()
        );
    }
}
impl Div<i64> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: i64) -> Self {
        //Self {
        //    x: self.x / _rhs as f32,
        //    y: self.y / _rhs as f32,
        //    z: self.z / _rhs as f32,
        //}
        return Vec3::from_simd4(
            self.to_simd4() / f32x4::from_array([_rhs as f32, _rhs as f32, _rhs as f32, 1.0])
        );
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: f32) -> Self {
        //Self {
        //    x: self.x / _rhs,
        //    y: self.y / _rhs,
        //    z: self.z / _rhs,
        //}
        return Vec3::from_simd4(
            self.to_simd4() / f32x4::from_array([_rhs, _rhs, _rhs, 1.0])
        );
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for i64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        //Vec3 {
        //    x: _rhs.x * self as f32,
        //    y: _rhs.y * self as f32,
        //    z: _rhs.z * self as f32,
        //}
        return Vec3::from_simd4(
            _rhs.to_simd4() * f32x4::from_array([self as f32, self as f32, self as f32, 1.0])
        );
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        //Vec3 {
        //    x: _rhs.x * self,
        //    y: _rhs.y * self,
        //    z: _rhs.z * self,
        //}
        return Vec3::from_simd4(
            _rhs.to_simd4() * f32x4::from_array([self, self, self, 1.0])
        );
    }
}
impl Div<Vec3> for i64 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        //Vec3 {
        //    x: _rhs.x / self as f32,
        //    y: _rhs.y / self as f32,
        //    z: _rhs.z / self as f32,
        //}
        return Vec3::from_simd4(
            _rhs.to_simd4() / f32x4::from_array([self as f32, self as f32, self as f32, 1.0])
        );
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        //Vec3 {
        //    x: _rhs.x / self,
        //    y: _rhs.y / self,
        //    z: _rhs.z / self,
        //}
        return Vec3::from_simd4(
            _rhs.to_simd4() / f32x4::from_array([self, self, self, 1.0])
        );
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f32 {
        return (self.to_simd4() * self.to_simd4())[0..3].iter().sum()
    }
    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn unit_vector(&self) -> Vec3 {
        return self.clone() / self.length();
    }

    pub fn dot_prod(&self, other: Vec3) -> f32 {
        //return self.x * other.x + self.y * other.y + self.z * other.z;
        return (self.to_simd4() * other.to_simd4())[0..3].iter().sum()
    }
    pub fn cross_prod(&self, other: Vec3) -> Vec3 {
        //return Vec3 {
        //    x: self.y * other.z - self.x * other.y,
        //    y: self.z * other.x - self.x * other.z,
        //    z: self.x * other.y - self.y * other.x,
        //};

        return Self::from_simd4(
            f32x4::from_array([self.y, self.z, self.x, 0.0]) *
                f32x4::from_array([other.z, other.x, other.y, 0.0]) -
                f32x4::from_array([self.x, self.x, self.y, 0.0]) *
                f32x4::from_array([other.y, other.z, other.x, 0.0])
        )
    }

    pub fn new(_x: f32, _y: f32, _z: f32) -> Vec3 {
        return Vec3 {
            x: _x,
            y: _y,
            z: _z,
        };
    }
    pub fn newi(_x: i64, _y: i64, _z: i64) -> Vec3 {
        return Vec3 {
            x: _x as f32,
            y: _y as f32,
            z: _z as f32,
        };
    }

    pub fn rand(rng: &mut ChaCha20Rng) -> Vec3 {
        return Vec3 {
            x: rand_double(rng),
            y: rand_double(rng),
            z: rand_double(rng),
        };
    }
    pub fn rand_range(rng: &mut ChaCha20Rng, min: f32, max: f32) -> Vec3 {
        return Vec3 {
            x: rand_double_range(rng, min, max),
            y: rand_double_range(rng, min, max),
            z: rand_double_range(rng, min, max),
        };
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x < s && self.y < s && self.z < s;
    }

    pub fn idx(&self, i: i8) -> f32 {
        let i = i % 3;
        if i == 0 {return self.x;}
        else if i == 1 {return self.y;}
        else {return self.z;}
    }


    pub fn to_simd4(&self) -> f32x4 {
        return f32x4::from_array([self.x, self.y, self.z, 1.0])
    }
    pub fn from_simd4(xyz_: f32x4) -> Self {
        let [x,y,z, _] = xyz_.as_array().to_owned();
        Self { x, y, z }
    }
}
