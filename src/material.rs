use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::vec3::{Color, Vec3, random_unit_vector, unit_vector, reflect, dot};

pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
	pub albedo: Color
}

impl Lambertian {
	pub fn new(albedo: Vec3) -> Self {
		Lambertian { albedo }
	}
}

impl Material for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		// let scatter_direction = rec.p + rec.normal + random_in_unit_sphere();
		let mut scatter_direction = rec.normal + random_unit_vector();
		// let scatter_direction = rec.p + random_in_hemisphere(&rec.normal);

		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}

		Some((self.albedo, Ray::new(rec.p, scatter_direction)))
	}
}

pub struct Metal {
	pub albedo: Color
}

impl Metal {
	pub fn new(albedo: Vec3) -> Self {
		Metal { albedo }
	}
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
		let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal);
		let scattered = Ray::new(rec.p, reflected);
		let attenuation = self.albedo;
		if dot(&scattered.direction(), &rec.normal) <= 0.0 {
			None
		} else {
			Some((attenuation, scattered))
		}
	}
}
