use std::rc::Rc;

use crate::{
	hittable::HitRecord,
	ray::Ray,
	texture::{SolidColor, Texture},
	util::random_f64,
	vec3::{
		Color, dot, Point3, random_in_unit_sphere, random_unit_vector, reflect, refract,
		unit_vector, Vec3,
	},
};

pub trait Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord,) -> Option<(Color, Ray,),>;

	fn emitted(&self, uu: f64, v: f64, p: &Point3,) -> Color { Color::new(0.0, 0.0, 0.0,) }
}

pub struct Lambertian {
	pub albedo: Rc<dyn Texture,>,
}

impl Lambertian {
	pub fn new(albedo: Color,) -> Self {
		Lambertian {
			albedo: Rc::new(SolidColor::from(albedo,),),
		}
	}
}

impl<T: 'static,> From<Rc<T,>,> for Lambertian
where T: Texture
{
	fn from(texture: Rc<T,>,) -> Self { Lambertian { albedo: texture, } }
}

impl Material for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord,) -> Option<(Color, Ray,),> {
		// let scatter_direction = rec.p + rec.normal + random_in_unit_sphere();
		let mut scatter_direction = rec.normal + random_unit_vector();
		// let scatter_direction = rec.p + random_in_hemisphere(&rec.normal);

		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}

		Some((
			self.albedo.value(rec.u, rec.v, &rec.p,),
			Ray::new(rec.p, scatter_direction, r_in.time(),),
		),)
	}
}

pub struct Metal {
	pub albedo: Color,
	pub fuzz:   f64,
}

impl Metal {
	pub fn new(albedo: Vec3, fuzz: f64,) -> Self { Metal { albedo, fuzz, } }
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord,) -> Option<(Vec3, Ray,),> {
		let reflected = reflect(&unit_vector(&r_in.direction(),), &rec.normal,);
		let scattered = Ray::new(
			rec.p,
			reflected + self.fuzz * random_unit_vector(),
			r_in.time(),
		);
		let attenuation = self.albedo;
		if dot(&scattered.direction(), &rec.normal,) <= 0.0 {
			None
		} else {
			Some((attenuation, scattered,),)
		}
	}
}

pub struct Dielectric {
	ir: f64,
}

impl Dielectric {
	pub fn new(ir: f64,) -> Self { Dielectric { ir, } }

	fn reflectance(cosine: f64, ref_idx: f64,) -> f64 {
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		r0 + (1.0 - r0) * (1.0 - cosine).powi(5,)
	}
}

impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord,) -> Option<(Vec3, Ray,),> {
		let refraction_ration = if rec.front_face {
			1.0 / self.ir
		} else {
			self.ir
		};

		let unit_direction = unit_vector(&r_in.dir,);
		let cos_theta = dot(&-unit_direction, &rec.normal,).min(1.0,);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

		let cannot_refract = refraction_ration * sin_theta > 1.0;
		let direction;

		if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ration,) > random_f64() {
			direction = reflect(&unit_direction, &rec.normal,);
		} else {
			direction = refract(&unit_direction, &rec.normal, refraction_ration,)
		}

		Some((
			Color::new(1.0, 1.0, 1.0,),
			Ray::new(rec.p, direction, r_in.time(),),
		),)
	}
}

pub struct DiffuseLight {
	emit: Rc<dyn Texture,>,
}

impl DiffuseLight {
	pub fn new(emit: Rc<dyn Texture,>,) -> Self { DiffuseLight { emit, } }
}

impl From<Color,> for DiffuseLight {
	fn from(color: Color,) -> Self {
		DiffuseLight {
			emit: Rc::new(SolidColor::from(color,),),
		}
	}
}

impl Material for DiffuseLight {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord,) -> Option<(Vec3, Ray,),> { None }

	fn emitted(&self, u: f64, v: f64, p: &Vec3,) -> Vec3 { self.emit.value(u, v, p,) }
}

pub struct Isotrophic {
	albedo: Rc<dyn Texture,>,
}

impl Isotrophic {
	pub fn new(albedo: Rc<dyn Texture,>,) -> Self { Isotrophic { albedo, } }
}

impl From<Color,> for Isotrophic {
	fn from(c: Color,) -> Self {
		Isotrophic {
			albedo: Rc::new(SolidColor::from(c,),),
		}
	}
}

impl Material for Isotrophic {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord,) -> Option<(Vec3, Ray,),> {
		Some((
			self.albedo.value(rec.u, rec.v, &rec.p,),
			Ray::new(rec.p, random_in_unit_sphere(), r_in.time(),),
		),)
	}
}
