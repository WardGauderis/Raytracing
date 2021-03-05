use std::f64::consts::PI;
use std::rc::Rc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct Sphere {
	pub center: Point3,
	pub radius: f64,
	pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
		Sphere {
			center,
			radius,
			mat_ptr,
		}
	}

	fn get_sphere_uv(p: &Point3) -> (f64, f64) {
		let theta = (-p.y()).acos();
		let phi = -p.z().atan2(p.x()) + PI;
		(phi / (2.0 * PI), theta / PI)
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
			u: 0.0,
			v: 0.0,
		});

		let outward_normal = (rec.as_ref().unwrap().p - self.center) / self.radius;
		let (u, v) = Sphere::get_sphere_uv(&outward_normal);
		let tmp = rec.as_mut().unwrap();
		tmp.u = u;
		tmp.v = v;
		tmp.set_face_normal(r, &outward_normal);

		rec
	}

	fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
		Some(AABB::new(
			self.center - Vec3::new(self.radius, self.radius, self.radius),
			self.center + Vec3::new(self.radius, self.radius, self.radius),
		))
	}
}
