use std::rc::Rc;

use crate::{
	aabb::AABB,
	hittable::{HitRecord, Hittable},
	material::{Isotrophic, Material},
	ray::Ray,
	texture::Texture,
	util::random_f64,
	vec3::{Color, Vec3},
};

pub struct ConstantMedium {
	boundary:        Rc<dyn Hittable,>,
	phase_function:  Rc<dyn Material,>,
	neg_inv_density: f64,
}

impl ConstantMedium {
	pub fn from_texture(boundary: Rc<dyn Hittable,>, d: f64, a: Rc<dyn Texture,>,) -> Self {
		ConstantMedium {
			boundary,
			phase_function: Rc::new(Isotrophic::new(a,),),
			neg_inv_density: -1.0 / d,
		}
	}

	pub fn from_color(boundary: Rc<dyn Hittable,>, d: f64, a: Color,) -> Self {
		ConstantMedium {
			boundary,
			phase_function: Rc::new(Isotrophic::from(a,),),
			neg_inv_density: -1.0 / d,
		}
	}
}

impl Hittable for ConstantMedium {
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64,) -> Option<HitRecord,> {
		const enable_debug: bool = false;
		let debugging = enable_debug && random_f64() < 0.00001;

		let mut rec1 = self.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY,)?;
		let mut rec2 = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY,)?;

		if debugging {
			eprintln!("\nt_min={}, t_max={}\n", rec1.t, rec2.t);
		}

		if rec1.t < t_min {
			rec1.t = t_min
		}
		if rec2.t > t_max {
			rec2.t = t_max
		}

		if rec1.t >= rec2.t {
			return None;
		}

		if rec1.t < 0.0 {
			rec1.t = 0.0;
		}

		let ray_length = r.direction().length();
		let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
		let hit_distance = self.neg_inv_density * random_f64().ln();

		if hit_distance < distance_inside_boundary {
			return None;
		}

		let t = rec1.t + hit_distance / ray_length;

		Some(HitRecord {
			p: r.at(t,),
			normal: Vec3::new(1.0, 0.0, 0.0,),
			mat_ptr: self.phase_function.clone(),
			t,
			u: 0.0,
			v: 0.0,
			front_face: true,
		},)
	}

	fn bounding_box(&self, time0: f64, time1: f64,) -> Option<AABB,> {
		self.boundary.bounding_box(time0, time1,)
	}
}
