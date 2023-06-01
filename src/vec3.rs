use std::ops::{Add, Div, Mul, Neg, Sub};

use rand_chacha::ChaCha20Rng;

use crate::rand_double::{rand_double, rand_double_range};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}
impl Mul<i64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: i64) -> Self {
        Self {
            x: self.x * _rhs as f64,
            y: self.y * _rhs as f64,
            z: self.z * _rhs as f64,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: f64) -> Self {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}
impl Div<Vec3> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: Self) -> Self {
        Self {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        }
    }
}
impl Div<i64> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: i64) -> Self {
        Self {
            x: self.x / _rhs as f64,
            y: self.y / _rhs as f64,
            z: self.z / _rhs as f64,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: f64) -> Self {
        Self {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
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
        Vec3 {
            x: _rhs.x * self as f64,
            y: _rhs.y * self as f64,
            z: _rhs.z * self as f64,
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x * self,
            y: _rhs.y * self,
            z: _rhs.z * self,
        }
    }
}
impl Div<Vec3> for i64 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x / self as f64,
            y: _rhs.y / self as f64,
            z: _rhs.z / self as f64,
        }
    }
}
impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: _rhs.x / self,
            y: _rhs.y / self,
            z: _rhs.z / self,
        }
    }
}

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn unit_vector(&self) -> Vec3 {
        return Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        } / self.length();
    }

    pub fn dot_prod(&self, other: Vec3) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
    pub fn cross_prod(&self, other: Vec3) -> Vec3 {
        return Vec3 {
            x: self.y * other.z - self.x * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
    }

    pub fn new(_x: f64, _y: f64, _z: f64) -> Vec3 {
        return Vec3 {
            x: _x,
            y: _y,
            z: _z,
        };
    }
    pub fn newi(_x: i64, _y: i64, _z: i64) -> Vec3 {
        return Vec3 {
            x: _x as f64,
            y: _y as f64,
            z: _z as f64,
        };
    }

    pub fn rand(rng: &mut ChaCha20Rng) -> Vec3 {
        return Vec3 {
            x: rand_double(rng),
            y: rand_double(rng),
            z: rand_double(rng),
        };
    }
    pub fn rand_range(rng: &mut ChaCha20Rng, min: f64, max: f64) -> Vec3 {
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

    pub fn idx(&self, i: i8) -> f64 {
        let i = i % 3;
        if i == 0 {return self.x;}
        else if i == 1 {return self.y;}
        else {return self.z;}
    }
}
