use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
	pub orig: Point3,
	pub dir: Vec3,
}

impl Ray {
	pub fn new(orig: Vec3, dir: Vec3) -> Self {
		Ray { orig, dir }
	}

	pub fn origin(&self) -> Point3 {
		self.orig
	}

	pub fn direction(&self) -> Vec3 {
		self.dir
	}

	pub fn at(&self, t: f64) -> Point3 {
		self.orig + t * self.dir
	}
}
