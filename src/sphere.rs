use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::rc::Rc;

pub struct Sphere {
	pub center: Point3,
	pub radius: f64,
	pub mat_ptr: Rc<Material>,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
		Sphere {
			center,
			radius,
			mat_ptr,
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = dot(&oc, &r.direction());
		let c = oc.length_squared() - self.radius * self.radius;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return None;
		}

		let sqrtd = discriminant.sqrt();

		let mut root = (-half_b - sqrtd) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrtd) / a;
			if root < t_min || t_max < root {
				return None;
			}
		}

		let mut rec = Some(HitRecord {
			p: r.at(root),
			t: root,
			normal: Default::default(),
			front_face: false,
			mat_ptr: self.mat_ptr.clone(),
		});
		let outward_normal = (rec.as_ref().unwrap().p - self.center) / self.radius;
		rec.as_mut().unwrap().set_face_normal(r, &outward_normal);

		rec
	}
}
