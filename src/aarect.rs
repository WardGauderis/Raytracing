use std::rc::Rc;

use crate::{
	aabb::AABB,
	hittable::{HitRecord, Hittable},
	material::Material,
	ray::Ray,
	vec3::{Point3, Vec3},
};

pub struct XYRect {
	mp: Rc<dyn Material,>,
	x0: f64,
	x1: f64,
	y0: f64,
	y1: f64,
	k:  f64,
}

impl XYRect {
	pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: Rc<dyn Material,>,) -> Self {
		XYRect {
			mp,
			x0,
			x1,
			y0,
			y1,
			k,
		}
	}
}

impl Hittable for XYRect {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		let t = (self.k - r.origin().z()) / r.direction().z();
		if t < t_min || t > t_max {
			return None;
		}
		let x = r.origin().x() + t * r.direction().x();
		let y = r.origin().y() + t * r.direction().y();
		if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
			return None;
		}

		let mut record = HitRecord {
			p: r.at(t,),
			normal: Default::default(),
			mat_ptr: self.mp.clone(),
			t,
			u: (x - self.x0) / (self.x1 - self.x0),
			v: (y - self.y0) / (self.y1 - self.y0),
			front_face: false,
		};
		let outward_normal = Vec3::new(0.0, 0.0, 1.0,);
		record.set_face_normal(r, &outward_normal,);

		Some(record,)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> {
		Some(AABB::new(
			Point3::new(self.x0, self.y0, self.k - 0.0001,),
			Point3::new(self.x1, self.y1, self.k + 0.0001,),
		),)
	}
}

pub struct XZRect {
	mp: Rc<dyn Material,>,
	x0: f64,
	x1: f64,
	z0: f64,
	z1: f64,
	k:  f64,
}

impl XZRect {
	pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mp: Rc<dyn Material,>,) -> Self {
		XZRect {
			mp,
			x0,
			x1,
			z0,
			z1,
			k,
		}
	}
}

impl Hittable for XZRect {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		let t = (self.k - r.origin().y()) / r.direction().y();
		if t < t_min || t > t_max {
			return None;
		}
		let x = r.origin().x() + t * r.direction().x();
		let z = r.origin().z() + t * r.direction().z();
		if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
			return None;
		}

		let mut record = HitRecord {
			p: r.at(t,),
			normal: Default::default(),
			mat_ptr: self.mp.clone(),
			t,
			u: (x - self.x0) / (self.x1 - self.x0),
			v: (z - self.z0) / (self.z1 - self.z0),
			front_face: false,
		};
		let outward_normal = Vec3::new(0.0, 1.0, 0.0,);
		record.set_face_normal(r, &outward_normal,);

		Some(record,)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> {
		Some(AABB::new(
			Point3::new(self.x0, self.k - 0.0001, self.z0,),
			Point3::new(self.x1, self.k + 0.0001, self.z1,),
		),)
	}
}

pub struct YZRect {
	mp: Rc<dyn Material,>,
	y0: f64,
	y1: f64,
	z0: f64,
	z1: f64,
	k:  f64,
}

impl YZRect {
	pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mp: Rc<dyn Material,>,) -> Self {
		YZRect {
			mp,
			y0,
			y1,
			z0,
			z1,
			k,
		}
	}
}

impl Hittable for YZRect {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		let t = (self.k - r.origin().x()) / r.direction().x();
		if t < t_min || t > t_max {
			return None;
		}
		let y = r.origin().y() + t * r.direction().y();
		let z = r.origin().z() + t * r.direction().z();
		if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
			return None;
		}

		let mut record = HitRecord {
			p: r.at(t,),
			normal: Default::default(),
			mat_ptr: self.mp.clone(),
			t,
			u: (y - self.y0) / (self.y1 - self.y0),
			v: (z - self.z0) / (self.z1 - self.z0),
			front_face: false,
		};
		let outward_normal = Vec3::new(1.0, 0.0, 0.0,);
		record.set_face_normal(r, &outward_normal,);

		Some(record,)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> {
		Some(AABB::new(
			Point3::new(self.k - 0.0001, self.y0, self.z0,),
			Point3::new(self.k + 0.0001, self.y1, self.z1,),
		),)
	}
}
