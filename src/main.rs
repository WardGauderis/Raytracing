use std::f64::INFINITY;
use std::fs::File;
use std::io::{stderr, Write};
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::random_f64;
use crate::vec3::{Color, Point3, random_in_hemisphere, random_in_unit_sphere, random_unit_vector, unit_vector};
use crate::material::{Lambertian, Metal};

mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod util;
mod material;


fn ray_color(r: &Ray, world: &impl Hittable, depth: i32) -> Color {
	if depth <= 0 {
		return Color::default();
	}

	if let Some(rec) = world.hit(r, 0.001, INFINITY) {
		if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
			return attenuation * ray_color(&scattered, world, depth - 1);
		}
		return Color::default();
	}
	let unit_direction = unit_vector(&r.direction());
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 100;
	let max_depth = 50;

	let mut world = HittableList::default();
	let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
	let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
	let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
	let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

	world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));

	let cam = Camera::new();

	let mut file = File::create("test.ppm").unwrap();

	write!(file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

	for j in (0..image_height).into_iter().rev() {
		eprintln!("\rScanlines remaining: {} ", j);
		for i in 0..image_width {
			let mut pixel_color = Color::default();
			for _ in 0..samples_per_pixel {
				let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
				let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
				let r = cam.get_ray(u, v);
				pixel_color += ray_color(&r, &world, max_depth);
			}
			write_color(&mut file, &pixel_color, samples_per_pixel);
		}
	}
	eprint!("\nDone\n");
}
