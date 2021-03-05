use crate::util::clamp;
use crate::vec3::Color;
use std::fs::File;
use std::io::Write;

pub fn write_color(file: &mut File, pixel_color: &Color, samples_per_pixel: i32) {
	let mut r = pixel_color.x();
	let mut g = pixel_color.y();
	let mut b = pixel_color.z();

	let scale = 1.0 / samples_per_pixel as f64;
	r = (scale * r).sqrt();
	g = (scale * g).sqrt();
	b = (scale * b).sqrt();
	write!(
		file,
		"{} {} {}\n",
		(255.99 * clamp(r, 0.0, 0.999)) as i32,
		(255.99 * clamp(g, 0.0, 0.999)) as i32,
		(255.99 * clamp(b, 0.0, 0.999)) as i32
	)
	.unwrap();
}
