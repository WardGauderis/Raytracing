use std::{
	cmp::{Ordering, Ordering::Less},
	rc::Rc,
};

use crate::{
	aabb::{AABB, surrounding_box},
	hittable::{HitRecord, Hittable},
	hittable_list::HittableList,
	ray::Ray,
	util::random_i32_in_range,
};

pub struct BVHNode {
	pub left:  Rc<dyn Hittable,>,
	pub right: Rc<dyn Hittable,>,
	pub aabb:  AABB,
}

impl BVHNode {
	pub fn from_list(list: &HittableList, time0: f64, time1: f64,) -> Self {
		BVHNode::new(&list.objects, 0, list.objects.len(), time0, time1,)
	}

	pub fn new(
		src_objects: &Vec<Rc<dyn Hittable,>,>,
		start: usize,
		end: usize,
		time0: f64,
		time1: f64,
	) -> Self {
		let mut objects = src_objects.clone();

		let axis = random_i32_in_range(0, 2,);
		let comparator = if axis == 0 {
			box_x_compare
		} else if axis == 1 {
			box_y_compare
		} else {
			box_z_compare
		};

		let left;
		let right;

		let object_span = end - start;

		if object_span == 1 {
			left = objects[start].clone();
			right = left.clone();
		} else if object_span == 2 {
			match comparator(&objects[start], &objects[start + 1],) {
				Ordering::Less => {
					left = objects[start].clone();
					right = objects[start + 1].clone();
				},
				_ => {
					left = objects[start + 1].clone();
					right = objects[start].clone();
				},
			}
		} else {
			objects.sort_by(comparator,);

			let mid = start + object_span / 2;
			left = Rc::new(BVHNode::new(&objects, start, mid, time0, time1,),);
			right = Rc::new(BVHNode::new(&objects, mid, end, time0, time1,),);
		}

		let aabb = surrounding_box(
			left.bounding_box(time0, time1,)
				.expect("No bounding box in BVHNode::new()",),
			right
				.bounding_box(time0, time1,)
				.expect("No bounding box in BVHNode::new()",),
		);

		BVHNode { left, right, aabb, }
	}
}

impl Hittable for BVHNode {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		if !self.aabb.hit(r, t_min, t_max,) {
			return None;
		}

		if let Some(hit_left,) = self.left.hit(r, t_min, t_max,) {
			self.right.hit(r, t_min, hit_left.t,).or(Some(hit_left,),)
		} else {
			self.right.hit(r, t_min, t_max,)
		}
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> { Some(self.aabb.clone(),) }
}

#[inline]
fn box_compare(a: &Rc<dyn Hittable,>, b: &Rc<dyn Hittable,>, axis: usize,) -> Ordering {
	let box_a = a
		.bounding_box(0.0, 0.0,)
		.expect("No bounding box in BVHNode::new()",);
	let box_b = b
		.bounding_box(0.0, 0.0,)
		.expect("No bounding box in BVHNode::new()",);

	box_a.min().e[axis]
		.partial_cmp(&box_b.min().e[axis],)
		.unwrap_or(Less,)
}

fn box_x_compare(a: &Rc<dyn Hittable,>, b: &Rc<dyn Hittable,>,) -> Ordering {
	box_compare(a, b, 0,)
}

fn box_y_compare(a: &Rc<dyn Hittable,>, b: &Rc<dyn Hittable,>,) -> Ordering {
	box_compare(a, b, 1,)
}

fn box_z_compare(a: &Rc<dyn Hittable,>, b: &Rc<dyn Hittable,>,) -> Ordering {
	box_compare(a, b, 2,)
}
