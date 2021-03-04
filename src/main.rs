use std::f64::INFINITY;
use std::fs::File;
use std::io::{stderr, Write};
use std::rc::Rc;

use crate::camera::Camera;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::material::{Lambertian, Metal, Dielectric, Material};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::util::{random_f64, random_f64_range};
use crate::vec3::{Color, Point3, random_in_hemisphere, random_in_unit_sphere, random_unit_vector, unit_vector, Vec3};
use std::f64::consts::PI;

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

fn random_scene() -> HittableList {
	let mut world = HittableList::default();

	let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
	world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = random_f64();
			let center = Point3::new(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());

			if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
				let sphere_material: Rc<dyn Material>;

				if choose_mat < 0.8 {
					let albedo = Color::random() * Color::random();
					sphere_material = Rc::new(Lambertian::new(albedo));
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
				} else if choose_mat < 0.95 {
					let albedo = Color::random_range(0.5, 1.0);
					let fuzz = random_f64_range(0.0, 0.5);
					sphere_material = Rc::new(Metal::new(albedo, fuzz));
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
				} else {
					sphere_material = Rc::new(Dielectric::new(1.5));
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
				}
			}
		}
	}

	let material1 = Rc::new(Dielectric::new(1.5));
	world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

	let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
	world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

	let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
	world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

	world
}

fn main() {
	let aspect_ratio = 3.0 / 2.0;
	let image_width = 1200;
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	let samples_per_pixel = 500;
	let max_depth = 50;

	let r = (PI / 4.0).cos();
	let world = random_scene();

	let lookfrom = Point3::new(13.0, 2.0, 3.0);
	let lookat = Point3::new(0.0, 0.0, 0.0);
	let vup = Vec3::new(0.0, 1.0, 0.0);
	let dist_to_focus = 10.0;
	let aperture = 0.1;

	let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus);

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
