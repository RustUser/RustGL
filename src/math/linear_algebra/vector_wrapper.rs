use std::ops::{Index, IndexMut, Sub};
use vecmath::{vec3_cross, vec3_dot, vec3_normalized};
use crate::math::linear_algebra::types::Vec3;

#[macro_export]
macro_rules! v3 {
    ($val:expr) => {
        Vec3Wrapper::new($val)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3Wrapper(pub Vec3);

impl Vec3Wrapper {
    pub fn to_owned(self) -> Vec3 {
        self.0
    }

    pub fn new(field0: Vec3) -> Self {
        Self(field0)
    }

    pub fn magnitude(&self) -> f32 {
        (self.0[0].powf(2.0) + self.0[1].powf(2.0) + self.0[2].powf(2.0)).sqrt()
    }

    pub fn normalize(&mut self) {
        *self = Self(vec3_normalized(self.0));
    }

    pub fn normalized(&self) -> Vec3Wrapper {
        let mut s = self.clone();
        s.normalize();
        s
    }

    pub fn cross(&self, b: &Vec3Wrapper) -> Vec3Wrapper {
        Self(vec3_cross(self.0, b.0))
    }

    pub fn dot(&self, b: &Vec3Wrapper) -> f32 {
        //vec3_dot(self.into(), b.into())
        vec3_dot(self.0, b.0)
    }

    pub fn x(&self) -> f32 {
        self[0]
    }

    pub fn y(&self) -> f32 {
        self[1]
    }

    pub fn z(&self) -> f32 {
        self[2]
    }
}

impl Into<Vec3> for Vec3Wrapper {
    fn into(self) -> Vec3 {
        self.to_owned()
    }
}

impl Index<usize> for Vec3Wrapper {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3Wrapper {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Sub<Vec3Wrapper> for Vec3Wrapper {
    type Output = Vec3Wrapper;

    fn sub(self, rhs: Vec3Wrapper) -> Self::Output {
        let a = self.0;
        let b = rhs.0;
        let mut c = [0f32; 3];
        for i in 0..a.len() {
            c[i] = a[i] - b[i];
        }
        Self(c)
    }
}