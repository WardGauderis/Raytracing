use std::rc::Rc;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
	pub objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
	pub fn new(object: Rc<dyn Hittable>) -> Self {
		HittableList { objects: vec![object] }
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
}

