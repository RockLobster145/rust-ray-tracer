use std::ops;
use std::ops::Mul;

#[derive(Clone, Debug, PartialEq)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3(x, y, z)
    }
    fn x(&self) -> f32 {
        self.0
    }
    fn y(&self) -> f32 {
        self.1
    }
    fn z(&self) -> f32 {
        self.2
    }
    fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, value: Self) {
        *self = Vec3(self.0 + value.0, self.1 + value.1, self.2 + value.2);
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, value: f32) {
        *self = Vec3(self.0 * value, self.1 * value, self.2 * value);
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, value: f32) {
        *self = Vec3(self.0 / value, self.1 / value, self.2 / value);
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, value: Vec3) -> Self {
        Vec3(self.0 + value.0, self.1 + value.1, self.2 + value.2)
    }
}
impl ops::Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, value: f32) -> Self {
        Vec3(self.0 + value, self.1 + value, self.2 + value)
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, value: Vec3) -> Self {
        Vec3(self.0 - value.0, self.1 - value.1, self.2 - value.2)
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul (self, value: f32) -> Self {
        Vec3(self.0 * value, self.1 * value, self.2 * value)
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, value: Vec3) -> Self {
        Vec3(self.0 * value.0, self.1 * value.1, self.2 * value.2)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec3: Vec3) -> Self::Output {
        vec3 * self
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, value: f32) -> Self {
        (1.0 / value) * self
    }
}


fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}
fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3(
        v1.1 * v2.2 - v1.2 * v2.1,
        v1.2 * v2.0 - v1.0 * v2.2,
        v1.0 * v2.1 - v1.1 * v2.0,
    )
}
fn unit_vector(v: Vec3) -> Vec3 {
    v.clone() / v.length()
}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_ve3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3(1.0, 2.0, 3.0));
    }
    #[test]
    fn x_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).x(), 1.0);
    }
    #[test]
    fn y_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).y(), 2.0);
    }
    #[test]
    fn z_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).z(), 3.0);
    }
    #[test]
    fn length_squared_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length_squared(), 14.0);
    }
    #[test]
    fn length_vec3() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).length(), 3.741657383);
    }
    #[test]
    fn neg_vec3() {
        assert_eq!(-Vec3(1.0, 2.0, 3.0), Vec3(-1.0, -2.0, -3.0));
    }
    #[test]
    fn add_assign_vec3_vec3() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        v += Vec3(1.0, 2.0, 3.0);
        assert_eq!(v, Vec3(2.0, 4.0, 6.0));
    }
    #[test]
    fn mul_assign_vec3_f32() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        v *= 2.0;
        assert_eq!(v, Vec3(2.0, 4.0, 6.0));
    }
    #[test]
    fn div_assign_vec3_f32() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, Vec3(0.5, 1.0, 1.5));
    }
    #[test]
    fn add_vec3_vec3() {
        let vec = Vec3(1.0, 2.0, 3.0);
        assert_eq!(Vec3(2.0, 4.0, 6.0), vec.clone() + vec);
    }
    #[test]
    fn add_vec3_f32() {
        let vec = Vec3(1.0, 2.0, 3.0);
        assert_eq!(Vec3(2.0, 3.0, 4.0), vec.clone() + 1.0);
        assert_eq!(Vec3(0.0, 1.0, 2.0), vec.clone() + -1.0);
    }
    #[test]
    fn sub_vec3_vec3() {
        assert_eq!(Vec3(1.0, 2.0, 3.0) - Vec3(1.0, 2.0, 3.0), Vec3(0.0, 0.0, 0.0));
    }
    #[test]
    fn mul_vec3_f32() {
        assert_eq!(Vec3(1.0, 2.0, 3.0) * 2.0, Vec3(2.0, 4.0, 6.0));
        assert_eq!(2.0 * Vec3(1.0, 2.0, 3.0), Vec3(2.0, 4.0, 6.0));
    }
    #[test]
    fn mul_vec3_vec3() {
        assert_eq!(Vec3(1.0, 2.0, 3.0) * Vec3(2.0, 4.0, 6.0), Vec3(2.0, 8.0, 18.0));
    }
    #[test]
    fn div_vec3_f32() {
        assert_eq!(Vec3(1.0, 2.0, 3.0) / 2.0, Vec3(0.5, 1.0, 1.5));
    }

    #[test]
    fn dot_vec3() {
        assert_eq!(dot(Vec3(1.0, 2.0, 3.0), Vec3(2.0, 4.0, 6.0)), 28.0);
    }
    #[test]
    fn cross_vec3() {
        assert_eq!(cross(Vec3(1.0, 2.0, 3.0), Vec3(2.0, 4.0, 6.0)), Vec3(0.0, 0.0, 0.0));
    }
    #[test]
    fn unit_vec3() {
        assert_eq!(unit_vector(Vec3(1.0, 2.0, 3.0)), Vec3(0.267261242, 0.534522484, 0.8017837))
    }
}