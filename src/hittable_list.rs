use crate::aabb::{AABB, surrounding_box};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
	pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
	pub fn new(object: Rc<dyn Hittable>) -> Self {
		HittableList {
			objects: vec![object],
		}
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add(&mut self, object: Rc<dyn Hittable>) {
		self.objects.push(object);
	}
}

impl Hittable for HittableList {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut temp_rec = None;
		let mut closes_so_far = t_max;

		for object in &self.objects {
			if let Some(rec) = object.hit(r, t_min, closes_so_far) {
				closes_so_far = rec.t;
				temp_rec.replace(rec);
			}
		}

		temp_rec
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
		if self.objects.is_empty() {
			return None;
		}

		let mut output_box = AABB::default();
		let mut first_box = true;

		for object in &self.objects {
			if let Some(temp_box) = object.bounding_box(time0, time1){
				output_box = if first_box {temp_box} else { surrounding_box(output_box, temp_box) };
				first_box = false;
			}
			return None;
		}

		Some(output_box)
	}
}
