use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

pub use Vec3 as Point3;
pub use Vec3 as Color;

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
	pub e: [f64; 3]
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self { e: [x, y, z] }
	}

	pub fn x(&self) -> f64 {
		self.e[0]
	}
	pub fn y(&self) -> f64 {
		self.e[1]
	}
	pub fn z(&self) -> f64 {
		self.e[2]
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn length_squared(&self) -> f64 {
		self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
	}
}

impl Neg for &Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Self::Output::new(-self.e[0], -self.e[1], -self.e[2])
	}
}

impl Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &Self::Output {
		&self.e[index]
	}
}

impl IndexMut<usize> for Vec3 {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.e[index]
	}
}

impl AddAssign<Vec3> for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		self.e[0] += rhs.e[0];
		self.e[1] += rhs.e[1];
		self.e[2] += rhs.e[2];
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, rhs: f64) {
		self.e[0] *= rhs;
		self.e[1] *= rhs;
		self.e[2] *= rhs;
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, rhs: f64) {
		*self *= 1.0 / rhs;
	}
}

impl Display for Vec3 {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
	}
}

impl Add for Vec3 {
	type Output = Vec3;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Self::Output::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		Self::Output::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
	}
}

impl Mul for Vec3 {
	type Output = Vec3;

	#[inline]
	fn mul(self, rhs: Self) -> Self::Output {
		Self::Output::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	#[inline]
	fn mul(self, rhs: Vec3) -> Self::Output {
		Self::Output::new(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2])
	}
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	#[inline]
	fn mul(self, rhs: f64) -> Self::Output {
		rhs * self
	}
}

impl Div<f64> for Vec3 {
	type Output = Vec3;

	#[inline]
	fn div(self, rhs: f64) -> Self::Output {
		1.0 / rhs * self
	}
}

#[inline]
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
	u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

#[inline]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	return Vec3::new(u.e[1] * v.e[2] - u.e[2] * v.e[1],
					 u.e[2] * v.e[0] - u.e[0] * v.e[2],
					 u.e[0] * v.e[1] - u.e[1] * v.e[0]);
}

#[inline]
pub fn unit_vector(v: &Vec3) -> Vec3 {
	*v / v.length()
}