use std::rc::Rc;

use crate::{
	aabb::AABB,
	aarect::{XYRect, XZRect, YZRect},
	hittable::{HitRecord, Hittable},
	hittable_list::HittableList,
	material::Material,
	ray::Ray,
	vec3::Point3,
};

pub struct Box {
	pub box_min: Point3,
	pub box_max: Point3,
	sides:       HittableList,
}

impl Box {
	pub fn new(p0: &Point3, p1: &Point3, ptr: Rc<dyn Material,>,) -> Self {
		let box_min = *p0;
		let box_max = *p1;

		let mut sides = HittableList::default();
		sides.add(Rc::new(XYRect::new(
			p0.x(),
			p1.x(),
			p0.y(),
			p1.y(),
			p1.z(),
			ptr.clone(),
		),),);
		sides.add(Rc::new(XYRect::new(
			p0.x(),
			p1.x(),
			p0.y(),
			p1.y(),
			p0.z(),
			ptr.clone(),
		),),);

		sides.add(Rc::new(XZRect::new(
			p0.x(),
			p1.x(),
			p0.z(),
			p1.z(),
			p1.y(),
			ptr.clone(),
		),),);
		sides.add(Rc::new(XZRect::new(
			p0.x(),
			p1.x(),
			p0.z(),
			p1.z(),
			p0.y(),
			ptr.clone(),
		),),);

		sides.add(Rc::new(YZRect::new(
			p0.y(),
			p1.y(),
			p0.z(),
			p1.z(),
			p1.x(),
			ptr.clone(),
		),),);
		sides.add(Rc::new(YZRect::new(
			p0.y(),
			p1.y(),
			p0.z(),
			p1.z(),
			p0.x(),
			ptr,
		),),);

		Box {
			box_min,
			box_max,
			sides,
		}
	}
}

impl Hittable for Box {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		self.sides.hit(r, t_min, t_max,)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> {
		Some(AABB::new(self.box_min, self.box_max,),)
	}
}
