use crate::ray::Ray;
use crate::util::{degrees_to_radians, random_f64_range};
use crate::vec3::{cross, random_in_unit_disk, unit_vector, Point3, Vec3};

pub struct Camera {
	origin: Point3,
	lower_left_corner: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	u: Vec3,
	v: Vec3,
	w: Vec3,
	lens_radius: f64,
	time0: f64,
	time1: f64,
}

impl Camera {
	pub fn new(
		lookfrom: Point3,
		lookat: Point3,
		vup: Vec3,
		vfov: f64,
		aspect_ratio: f64,
		aperture: f64,
		focus_dis: f64,
		time0: f64,
		time1: f64,
	) -> Self {
		let theta = degrees_to_radians(vfov);
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = unit_vector(&(lookfrom - lookat));
		let u = unit_vector(&cross(&vup, &w));
		let v = cross(&w, &u);

		let origin = lookfrom;
		let horizontal = focus_dis * viewport_width * u;
		let vertical = focus_dis * viewport_height * v;
		let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dis * w;
		let lens_radius = aperture / 2.0;

		Camera {
			origin,
			horizontal,
			vertical,
			u,
			v,
			w,
			lower_left_corner,
			lens_radius,
			time0,
			time1,
		}
	}

	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		let rd = self.lens_radius * random_in_unit_disk();
		let offset = self.u * rd.x() + self.v * rd.y();
		Ray::new(
			self.origin + offset,
			self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
			random_f64_range(self.time0, self.time1),
		)
	}
}
