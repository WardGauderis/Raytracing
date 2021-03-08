#![feature(destructuring_assignment)]
use std::{
	f64::INFINITY,
	fs::File,
	io::{stderr, Write},
	rc::Rc,
	time::{Duration, Instant},
};

use crate::{
	aarect::XYRect,
	bvh::BVHNode,
	camera::Camera,
	color::write_color,
	hittable::Hittable,
	hittable_list::HittableList,
	material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
	movingsphere::MovingSphere,
	ray::Ray,
	sphere::Sphere,
	texture::{CheckerTexture, ImageTexture, NoiseTexture},
	util::{random_f64, random_f64_range},
	vec3::{unit_vector, Color, Point3, Vec3},
};

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod movingsphere;
mod perlin;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32,) -> Color {
	if depth <= 0 {
		return Color::default();
	}

	if let Some(rec,) = world.hit(r, 0.001, INFINITY,) {
		let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &rec.p,);
		if let Some((attenuation, scattered,),) = rec.mat_ptr.scatter(r, &rec,) {
			return emitted + attenuation * ray_color(&scattered, background, world, depth - 1,);
		}
		return emitted;
	}

	*background
}

fn random_scene() -> HittableList {
	let mut world = HittableList::default();

	let checker = Rc::new(CheckerTexture::new(
		Color::new(0.2, 0.3, 0.1,),
		Color::new(0.9, 0.9, 0.9,),
	),);
	let ground_material = Rc::new(Lambertian::from(checker,),);
	world.add(Rc::new(Sphere::new(
		Point3::new(0.0, -1000.0, 0.0,),
		1000.0,
		ground_material,
	),),);

	for a in -11 .. 11 {
		for b in -11 .. 11 {
			let choose_mat = random_f64();
			let center = Point3::new(
				a as f64 + 0.9 * random_f64(),
				0.2,
				b as f64 + 0.9 * random_f64(),
			);

			if (center - Point3::new(4.0, 0.2, 0.0,)).length() > 0.9 {
				let sphere_material: Rc<dyn Material,>;

				if choose_mat < 0.8 {
					let albedo = Color::random() * Color::random();
					sphere_material = Rc::new(Lambertian::new(albedo,),);
					let center2 = center + Vec3::new(0.0, random_f64_range(0.0, 0.5,), 0.0,);
					world.add(Rc::new(MovingSphere::new(
						center,
						center2,
						0.0,
						1.0,
						0.2,
						sphere_material,
					),),);
				} else if choose_mat < 0.95 {
					let albedo = Color::random_in_range(0.5, 1.0,);
					let fuzz = random_f64_range(0.0, 0.5,);
					sphere_material = Rc::new(Metal::new(albedo, fuzz,),);
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material,),),);
				} else {
					sphere_material = Rc::new(Dielectric::new(1.5,),);
					world.add(Rc::new(Sphere::new(center, 0.2, sphere_material,),),);
				}
			}
		}
	}

	let material1 = Rc::new(Dielectric::new(1.5,),);
	world.add(Rc::new(Sphere::new(
		Point3::new(0.0, 1.0, 0.0,),
		1.0,
		material1,
	),),);

	let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1,),),);
	world.add(Rc::new(Sphere::new(
		Point3::new(-4.0, 1.0, 0.0,),
		1.0,
		material2,
	),),);

	let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5,), 0.0,),);
	world.add(Rc::new(Sphere::new(
		Point3::new(4.0, 1.0, 0.0,),
		1.0,
		material3,
	),),);

	world
}

fn two_spheres() -> HittableList {
	let mut objects = HittableList::default();

	let checker = Rc::new(CheckerTexture::new(
		Color::new(0.2, 0.3, 0.1,),
		Color::new(0.9, 0.9, 0.9,),
	),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, -10.0, 0.0,),
		10.0,
		Rc::new(Lambertian::from(checker.clone(),),),
	),),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, 10.0, 0.0,),
		10.0,
		Rc::new(Lambertian::from(checker,),),
	),),);

	objects
}

fn two_perlin_spheres() -> HittableList {
	let mut objects = HittableList::default();

	let pertext = Rc::new(NoiseTexture::new(4.0,),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, -1000.0, 0.0,),
		1000.0,
		Rc::new(Lambertian::from(pertext.clone(),),),
	),),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, 2.0, 0.0,),
		2.0,
		Rc::new(Lambertian::from(pertext,),),
	),),);

	objects
}

fn earth() -> HittableList {
	let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg",),);
	let earth_surface = Rc::new(Lambertian::from(earth_texture,),);
	let globe = Rc::new(Sphere::new(Point3::default(), 2.0, earth_surface,),);

	HittableList::new(globe,)
}

fn simple_light() -> HittableList {
	let mut objects = HittableList::default();

	let pertext = Rc::new(NoiseTexture::new(4.0,),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, -1000.0, 0.0,),
		1000.0,
		Rc::new(Lambertian::from(pertext.clone(),),),
	),),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, 2.0, 0.0,),
		2.0,
		Rc::new(Lambertian::from(pertext,),),
	),),);

	let difflight = Rc::new(DiffuseLight::from(Color::new(4.0, 4.0, 4.0,),),);
	objects.add(Rc::new(XYRect::new(
		3.0,
		5.0,
		1.0,
		3.0,
		-2.0,
		difflight.clone(),
	),),);
	objects.add(Rc::new(Sphere::new(
		Point3::new(0.0, 7.0, 0.0,),
		2.0,
		difflight,
	),),);

	objects
}

fn main() {
	let aspect_ratio = 16.0 / 9.0;
	let image_width = 400;
	let image_height = (image_width as f64 / aspect_ratio) as i32;
	let mut samples_per_pixel = 100;
	let max_depth = 50;

	let world;
	let lookfrom;
	let lookat;
	let mut vfov = 40.0;
	let mut aperture = 0.0;
	let mut background = Color::default();

	match 0 {
		1 => {
			world = random_scene();
			background = Color::new(0.70, 0.80, 1.00,);
			lookfrom = Point3::new(13.0, 2.0, 3.0,);
			lookat = Point3::new(0.0, 0.0, 0.0,);
			vfov = 20.0;
			aperture = 0.1;
		},
		2 => {
			world = two_spheres();
			background = Color::new(0.70, 0.80, 1.00,);
			lookfrom = Point3::new(13.0, 2.0, 3.0,);
			lookat = Point3::new(0.0, 0.0, 0.0,);
			vfov = 20.0;
		},
		3 => {
			world = two_perlin_spheres();
			background = Color::new(0.70, 0.80, 1.00,);
			lookfrom = Point3::new(13.0, 2.0, 3.0,);
			lookat = Point3::new(0.0, 0.0, 0.0,);
			vfov = 20.0;
		},
		4 => {
			world = earth();
			background = Color::new(0.70, 0.80, 1.00,);
			lookfrom = Point3::new(13.0, 2.0, 3.0,);
			lookat = Point3::default();
			vfov = 20.0;
		},
		_ => {
			world = simple_light();
			samples_per_pixel = 400;
			background = Color::default();
			lookfrom = Point3::new(26.0, 3.0, 6.0,);
			lookat = Point3::new(0.0, 2.0, 0.0,);
			vfov = 20.0;
		},
	}

	let world = BVHNode::from_list(&world, 0.0, 1.0,);

	let vup = Vec3::new(0.0, 1.0, 0.0,);
	let dist_to_focus = 10.0;

	let cam = Camera::new(
		lookfrom,
		lookat,
		vup,
		20.0,
		aspect_ratio,
		aperture,
		dist_to_focus,
		0.0,
		1.0,
	);

	let mut file = File::create("test.ppm",).unwrap();

	write!(file, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

	let mut time_per_line = 0.0;
	let mut max = 0.0;

	for j in (0 .. image_height).into_iter().rev() {
		let now = Instant::now();

		for i in 0 .. image_width {
			let mut pixel_color = Color::default();
			for _ in 0 .. samples_per_pixel {
				let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
				let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
				let r = cam.get_ray(u, v,);
				pixel_color += ray_color(&r, &background, &world, max_depth,);
			}
			write_color(&mut file, &pixel_color, samples_per_pixel,);
		}

		time_per_line = 0.9 * time_per_line + 0.1 * now.elapsed().as_secs_f64();
		max = (time_per_line * j as f64).max(max,);
		eprint!("\r{} scanlines; {}s", j, time_per_line * j as f64);
		stderr().flush();
	}

	eprint!("\nDone\n{}s", max);
}
