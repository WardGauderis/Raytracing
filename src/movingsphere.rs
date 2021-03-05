use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::rc::Rc;

pub struct MovingSphere {
	pub center0: Point3,
	pub center1: Point3,
	pub time0: f64,
	pub time1: f64,
	pub radius: f64,
	pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
	pub fn new(
		center0: Point3,
		center1: Point3,
		time0: f64,
		time1: f64,
		radius: f64,
		mat_ptr: Rc<dyn Material>,
	) -> Self {
		MovingSphere {
			center0,
			center1,
			time0,
			time1,
			radius,
			mat_ptr,
		}
	}

	pub fn center(&self, time: f64) -> Point3 {
		self.center0
			+ ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
	}
}

impl Hittable for MovingSphere {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = r.origin() - self.center(r.time());
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
			u: 0.0,
			v: 0.0
		});
		let outward_normal = (rec.as_ref().unwrap().p - self.center(r.time())) / self.radius;
		rec.as_mut().unwrap().set_face_normal(r, &outward_normal);

		rec
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
		let box0 = AABB::new(
			self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
			self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
		);
		let box1 = AABB::new(
			self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
			self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
		);

		Some(surrounding_box(box0, box1))
	}
}
