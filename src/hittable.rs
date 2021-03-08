use std::rc::Rc;

use crate::{
	aabb::AABB,
	material::Material,
	ray::Ray,
	util::degrees_to_radians,
	vec3::{dot, Point3, Vec3},
};

pub struct HitRecord {
	pub p:          Point3,
	pub normal:     Vec3,
	pub mat_ptr:    Rc<dyn Material,>,
	pub t:          f64,
	pub u:          f64,
	pub v:          f64,
	pub front_face: bool,
}

impl HitRecord {
	#[inline]
	pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3,) {
		self.front_face = dot(&r.direction(), &outward_normal,) < 0.0;
		self.normal = if self.front_face {
			*outward_normal
		} else {
			-outward_normal
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,>;

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,>;
}

pub struct Translate {
	ptr:    Rc<dyn Hittable,>,
	offset: Vec3,
}

impl Translate {
	pub fn new(ptr: Rc<dyn Hittable,>, offset: Vec3,) -> Self { Translate { ptr, offset, } }
}

impl Hittable for Translate {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time(),);
		self.ptr.hit(&moved_r, t_min, t_max,).map(|mut rec| {
			rec.p += self.offset;
			rec.set_face_normal(&moved_r, &rec.normal.clone(),);
			rec
		},)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> {
		self.ptr.bounding_box(time0, time1,).map(|mut b| {
			b.minimum += self.offset;
			b.maximum += self.offset;
			b
		},)
	}
}

pub struct RotateY {
	ptr:       Rc<dyn Hittable,>,
	sin_theta: f64,
	cos_theta: f64,
	bbox:      Option<AABB,>,
}

impl RotateY {
	pub fn new(ptr: Rc<dyn Hittable,>, angle: f64,) -> Self {
		let radians = degrees_to_radians(angle,);
		let sin_theta = radians.sin();
		let cos_theta = radians.cos();

		match ptr.bounding_box(0.0, 1.0,) {
			None => RotateY {
				ptr,
				sin_theta,
				cos_theta,
				bbox: None,
			},
			Some(mut bbox,) => {
				let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY,);
				let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY,);

				for i in 0 .. 2 {
					for j in 0 .. 2 {
						for k in 0 .. 2 {
							let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
							let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
							let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

							let newx = cos_theta * x + sin_theta * z;
							let newz = -sin_theta * x + cos_theta * z;

							let tester = Vec3::new(newx, y, newz,);

							for c in 0 .. 3 {
								min[c] = min[c].min(tester[c],);
								max[c] = max[c].max(tester[c],);
							}
						}
					}
				}

				bbox = AABB::new(min, max,);

				RotateY {
					ptr,
					sin_theta,
					cos_theta,
					bbox: Some(bbox,),
				}
			},
		}
	}
}

impl Hittable for RotateY {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		let mut origin = r.origin();
		let mut direction = r.direction();

		origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
		origin[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.origin()[2];

		direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
		direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

		let rotated_r = Ray::new(origin, direction, r.time(),);

		self.ptr.hit(&rotated_r, t_min, t_max,).map(|mut rec| {
			let mut p = rec.p;
			let mut normal = rec.normal;

			p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
			p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

			normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
			normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

			rec.p = p;
			rec.set_face_normal(&rotated_r, &normal,);

			rec
		},)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> { self.bbox.clone() }
}
