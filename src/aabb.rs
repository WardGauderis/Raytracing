use std::mem::swap;

use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Default, Clone)]
pub struct AABB {
	pub minimum: Point3,
	pub maximum: Point3,
}

impl AABB {
	pub fn new(minimum: Point3, maximum: Point3) -> Self {
		AABB { minimum, maximum }
	}

	pub fn min(&self) -> Point3 {
		self.minimum
	}

	pub fn max(&self) -> Point3 {
		self.maximum
	}

	pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
		for a in 0..3 {
			let inv_d = 1.0 / r.direction()[a];
			let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
			let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;
			if inv_d < 0.0 {
				swap(&mut t0, &mut t1);
			}
			t_min = if t0 > t_min { t0 } else { t_min };
			t_max = if t1 < t_max { t1 } else { t_max };

			if t_max <= t_min {
				return false;
			}
		}
		true
	}
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
	let small = Point3::new(
		box0.min().x().min(box1.min().x()),
		box0.min().y().min(box1.min().y()),
		box0.min().z().min(box1.min().z()),
	);

	let big = Point3::new(
		box0.max().x().max(box1.max().x()),
		box0.max().y().max(box1.max().y()),
		box0.max().z().max(box1.max().z()),
	);

	AABB::new(small, big)
}
