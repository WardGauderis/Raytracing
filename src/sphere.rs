use crate::vec3::{Point3, Vec3, dot};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

#[derive(Default)]
pub struct Sphere {
	pub center: Point3,
	pub radius: f64,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64) -> Self {
		Sphere { center, radius }
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
		});
		let outward_normal = (rec.as_ref().unwrap().p - self.center) / self.radius;
		rec.as_mut().unwrap().set_face_normal(r, &outward_normal);

		rec
	}
}