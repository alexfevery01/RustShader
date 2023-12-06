use std::ops::{Add, AddAssign, Div, Mul, Sub,Index, IndexMut};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new<T: IntoVec2>(input: T) -> Vec2 {
        input.into_vec2()
    }

    // Getters for xy and yx
    pub fn xy(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn yx(&self) -> Vec2 {
        Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    // Setters for xy and yx
    pub fn set_xy(&mut self, value: Vec2) {
        self.x = value.x;
        self.y = value.y;
    }

    pub fn set_yx(&mut self, value: Vec2) {
        self.x = value.y;
        self.y = value.x;
    }

	 pub fn zero() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    // Sin operation
    pub fn sin(&self) -> Vec2 {
        Vec2::new((self.x.sin(), self.y.sin()))
    }

	pub fn abs(&self) -> Self {
        Vec2::new((self.x.abs(), self.y.abs()))
    }

    // Cos operation
    pub fn cos(&self) -> Vec2 {
        Vec2::new((self.x.cos(), self.y.cos()))
    }

    // Mix operation
    pub fn mix(in1: Vec2, in2: Vec2, factor: f32) -> Vec2 {
        Vec2::new((
            in1.x + (in2.x - in1.x) * factor,
            in1.y + (in2.y - in1.y) * factor,
        ))
    }

    //normalize
    pub fn normalize(input: Vec2) -> Vec2 {
        let len = input.length();
        if len > 0.0 {
            Vec2::new((input.x / len, input.y / len))
        } else {
            Vec2::new((0.0, 0.0))
        }
    }

    // Floor operation
    pub fn floor(&self) -> Vec2 {
        Vec2::new((self.x.floor(), self.y.floor()))
    }

    // dot product
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    // length/magnitude of the vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // fract operation to get the fractional part of a number
    pub fn fract(&self) -> Vec2 {
        Vec2::new((self.x.fract(), self.y.fract()))
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new((self.x + other.x, self.y + other.y))
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new((self.x - other.x, self.y - other.y))
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new((self.x * other.x, self.y * other.y))
    }
}

impl Div for Vec2 {
    type Output = Vec2;

    fn div(self, other: Vec2) -> Vec2 {
        Vec2::new((self.x / other.x, self.y / other.y))
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, other: f32) -> Vec2 {
        Vec2::new((self.x + other, self.y + other))
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: f32) -> Vec2 {
        Vec2::new((self.x - other, self.y - other))
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, other: f32) -> Vec2 {
        Vec2::new((self.x * other, self.y * other))
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, other: f32) -> Vec2 {
        Vec2::new((self.x / other, self.y / other))
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new((other.x * self, other.y * self))
    }
}

impl Add<Vec2> for f32 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new((other.x + self, other.y + self))
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new<T: IntoVec3>(input: T) -> Vec3 {
        input.into_vec3()
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn set_xyz(&mut self, value: Vec3) {
        self.x = value.x;
        self.y = value.y;
        self.z = value.z;
    }

	pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn sin(&self) -> Vec3 {
        Vec3::new((self.x.sin(), self.y.sin(), self.z.sin()))
    }

    pub fn cos(&self) -> Vec3 {
        Vec3::new((self.x.cos(), self.y.cos(), self.z.cos()))
    }

    pub fn mix(in1: Vec3, in2: Vec3, factor: f32) -> Vec3 {
        Vec3::new((
            in1.x + (in2.x - in1.x) * factor,
            in1.y + (in2.y - in1.y) * factor,
            in1.z + (in2.z - in1.z) * factor,
        ))
    }

    pub fn floor(&self) -> Vec3 {
        Vec3::new((self.x.floor(), self.y.floor(), self.z.floor()))
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn fract(&self) -> Vec3 {
        Vec3::new((self.x.fract(), self.y.fract(), self.z.fract()))
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn distance(&self, other: &Vec3) -> f32 {
        (*self - *other).length()
    }

    pub fn pow(&self, exponent: Vec3) -> Vec3 {
        Vec3 {
            x: self.x.powf(exponent.x),
            y: self.y.powf(exponent.y),
            z: self.z.powf(exponent.z),
        }
    }

    pub fn max(v1: Vec3, v2: Vec3) -> Vec3 {
        if v1.length() > v2.length() {
            v1
        } else {
            v2
        }
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - 2.0 * Vec3::dot(&normal, self) * normal
    }

    pub fn normalize(input: Vec3) -> Vec3 {
        let len = input.length();
        if len > 0.0 {
            Vec3::new((input.x / len, input.y / len, input.z / len))
        } else {
            Vec3::new((0.0, 0.0, 0.0))
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}



impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new((self.x + other.x, self.y + other.y, self.z + other.z))
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new((self.x - other.x, self.y - other.y, self.z - other.z))
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new((self.x * other.x, self.y * other.y, self.z * other.z))
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new((self.x / other.x, self.y / other.y, self.z / other.z))
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3::new((self.x + other, self.y + other, self.z + other))
    }
}

impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3::new((self.x - other, self.y - other, self.z - other))
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new((self.x * other, self.y * other, self.z * other))
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new((self.x / other, self.y / other, self.z / other))
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new((other.x * self, other.y * self, other.z * self))
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new((other.x + self, other.y + self, other.z + self))
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new<T: IntoVec4>(input: T) -> Vec4 {
        input.into_vec4()
    }

    pub fn xyz(&self) -> Vec3 {
        Vec3::new((self.x, self.y, self.z))
    }

    pub fn xz(&self) -> Vec2 {
        Vec2::new((self.x, self.z))
    }

    pub fn xy(&self) -> Vec2 {
        Vec2::new((self.x, self.y))
    }

    pub fn xw(&self) -> Vec2 {
        Vec2::new((self.x, self.w))
    }

    pub fn yz(&self) -> Vec2 {
        Vec2::new((self.y, self.z))
    }

    pub fn yw(&self) -> Vec2 {
        Vec2::new((self.y, self.w))
    }

    pub fn zw(&self) -> Vec2 {
        Vec2::new((self.z, self.w))
    }

	 pub fn zero() -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn fract(&self) -> Vec4 {
        Vec4 {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
            w: self.w.fract(),
        }
    }

    pub fn dot(a: &Vec4, b: &Vec4) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    pub fn sin(&self) -> Vec4 {
        Vec4::new((self.x.sin(), self.y.sin(), self.z.sin(), self.w.sin()))
    }

    pub fn cos(&self) -> Vec4 {
        Vec4::new((self.x.cos(), self.y.cos(), self.z.cos(), self.w.cos()))
    }

    pub fn mix(in1: Vec4, in2: Vec4, factor: f32) -> Vec4 {
        Vec4::new((
            in1.x + (in2.x - in1.x) * factor,
            in1.y + (in2.y - in1.y) * factor,
            in1.z + (in2.z - in1.z) * factor,
            in1.w + (in2.w - in1.w) * factor,
        ))
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds"),
        }
    }
}


impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4::new((
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        ))
    }
}

impl AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4::new((
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        ))
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, other: f32) -> Vec4 {
        Vec4::new((
            self.x * other,
            self.y * other,
            self.z * other,
            self.w * other,
        ))
    }
}

impl Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new((
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        ))
    }
}

impl Add<f32> for Vec4 {
    type Output = Vec4;

    fn add(self, other: f32) -> Vec4 {
        Vec4::new((
            self.x + other,
            self.y + other,
            self.z + other,
            self.w + other,
        ))
    }
}

impl Add<Vec4> for f32 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4::new((
            self + other.x,
            self + other.y,
            self + other.z,
            self + other.w,
        ))
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, other: f32) -> Vec4 {
        Vec4::new((
            self.x / other,
            self.y / other,
            self.z / other,
            self.w / other,
        ))
    }
}

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec4::new((
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        ))
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new((
            other.x * self,
            other.y * self,
            other.z * self,
            other.w * self,
        ))
    }
}

pub trait IntoVec2 {
    fn into_vec2(self) -> Vec2;
}

impl IntoVec2 for f32 {
    fn into_vec2(self) -> Vec2 {
        Vec2 { x: self, y: self }
    }
}

impl IntoVec2 for (f32, f32) {
    fn into_vec2(self) -> Vec2 {
        Vec2 {
            x: self.0,
            y: self.1,
        }
    }
}

pub trait IntoVec3 {
    fn into_vec3(self) -> Vec3;
}

impl IntoVec3 for f32 {
    fn into_vec3(self) -> Vec3 {
        Vec3 {
            x: self,
            y: self,
            z: self,
        }
    }
}

impl IntoVec3 for (Vec2, f32) {
    fn into_vec3(self) -> Vec3 {
        Vec3 {
            x: self.0.x,
            y: self.0.y,
            z: self.1,
        }
    }
}

impl IntoVec3 for (f32, Vec2) {
    fn into_vec3(self) -> Vec3 {
        Vec3 {
            x: self.0,
            y: self.1.x,
            z: self.1.y,
        }
    }
}

impl IntoVec3 for (f32, f32, f32) {
    fn into_vec3(self) -> Vec3 {
        Vec3 {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}

pub trait IntoVec4 {
    fn into_vec4(self) -> Vec4;
}

impl IntoVec4 for (Vec3, f32) {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.0.z,
            w: self.1,
        }
    }
}

impl IntoVec4 for (f32, f32, f32, f32) {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self.0,
            y: self.1,
            z: self.2,
            w: self.3,
        }
    }
}

impl IntoVec4 for (Vec2, Vec2) {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.1.x,
            w: self.1.y,
        }
    }
}

impl IntoVec4 for (Vec2, f32, f32) {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self.0.x,
            y: self.0.y,
            z: self.1,
            w: self.2,
        }
    }
}

impl IntoVec4 for (f32, Vec3) {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self.0,
            y: self.1.x,
            z: self.1.y,
            w: self.1.z,
        }
    }
}

impl IntoVec4 for (f32, f32, Vec2) {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self.0,
            y: self.1,
            z: self.2.x,
            w: self.2.y,
        }
    }
}

impl IntoVec4 for f32 {
    fn into_vec4(self) -> Vec4 {
        Vec4 {
            x: self,
            y: self,
            z: self,
            w: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_sin() {
        let v = Vec2::new((0.0, 1.0));
        let result = v.sin();
        assert_eq!(result, Vec2::new((0.0, 0.8414709848078965)));
    }

    #[test]
    fn test_vec2_cos() {
        let v = Vec2::new((0.0, 1.0));
        let result = v.cos();
        assert_eq!(result, Vec2::new((1.0, 0.5403023058681398)));
    }

    #[test]
    fn test_vec2_mix() {
        let v1 = Vec2::new((0.0, 1.0));
        let v2 = Vec2::new((1.0, 2.0));
        let factor = 0.5;
        let result = Vec2::mix(v1, v2, factor);
        assert_eq!(result, Vec2::new((0.5, 1.5)));
    }

    #[test]
    fn test_vec2_floor() {
        let v = Vec2::new((1.5, 2.7));
        let result = v.floor();
        assert_eq!(result, Vec2::new((1.0, 2.0)));
    }

    #[test]
    fn test_vec3_sin() {
        let v = Vec3::new((0.0, 1.0, 2.0));
        let result = v.sin();
        assert_eq!(
            result,
            Vec3::new((0.0, 0.8414709848078965, 0.9092974268256817))
        );
    }

    #[test]
    fn test_vec3_cross() {
        let v1 = Vec3::new((1.0, 0.0, 0.0));
        let v2 = Vec3::new((0.0, 1.0, 0.0));
        let result = v1.cross(&v2);
        assert_eq!(result, Vec3::new((0.0, 0.0, 1.0)));
    }

    #[test]
    fn test_vec3_distance() {
        let v1 = Vec3::new((1.0, 2.0, 3.0));
        let v2 = Vec3::new((4.0, 5.0, 6.0));
        let result = v1.distance(&v2);
        assert_eq!(result, 5.196152422706632);
    }

    #[test]
    fn test_vec3_pow() {
        let v = Vec3::new((2.0, 3.0, 4.0));
        let exponent = Vec3::new((2.0, 2.0, 2.0));
        let result = v.pow(exponent);
        assert_eq!(result, Vec3::new((4.0, 9.0, 16.0)));
    }

    #[test]
    fn test_vec3_reflect() {
        let v = Vec3::new((1.0, 1.0, 1.0));
        let normal = Vec3::new((0.0, 1.0, 0.0));
        let result = v.reflect(normal);
        assert_eq!(result, Vec3::new((1.0, -1.0, 1.0)));
    }
}
