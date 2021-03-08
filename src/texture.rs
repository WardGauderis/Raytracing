use std::rc::Rc;

use image::{GenericImageView, open};

use crate::{
	perlin::Perlin,
	util::clamp,
	vec3::{Color, Point3, Vec3},
};

pub trait Texture {
	fn value(&self, u: f64, v: f64, p: &Point3,) -> Color;
}

#[derive(Default,)]
pub struct SolidColor {
	color_value: Color,
}

impl SolidColor {
	pub fn new(red: f64, green: f64, blue: f64,) -> Self {
		SolidColor {
			color_value: Color::new(red, green, blue,),
		}
	}
}

impl From<Color,> for SolidColor {
	fn from(color: Color,) -> Self {
		SolidColor {
			color_value: color,
		}
	}
}

impl Texture for SolidColor {
	fn value(&self, u: f64, v: f64, p: &Point3,) -> Color { self.color_value }
}

pub struct CheckerTexture {
	even: Rc<dyn Texture,>,
	odd:  Rc<dyn Texture,>,
}

impl CheckerTexture {
	pub fn new(even: Color, odd: Color,) -> Self {
		CheckerTexture {
			even: Rc::new(SolidColor::from(even,),),
			odd:  Rc::new(SolidColor::from(odd,),),
		}
	}
}

impl Texture for CheckerTexture {
	fn value(&self, u: f64, v: f64, p: &Point3,) -> Color {
		let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
		if sines < 0.0 {
			self.odd.value(u, v, p,)
		} else {
			self.even.value(u, v, p,)
		}
	}
}

pub struct NoiseTexture {
	noise: Perlin,
	scale: f64,
}

impl NoiseTexture {
	pub fn new(scale: f64,) -> Self {
		NoiseTexture {
			noise: Perlin::new(),
			scale,
		}
	}
}

impl Texture for NoiseTexture {
	fn value(&self, u: f64, v: f64, p: &Vec3,) -> Vec3 {
		Color::new(1.0, 1.0, 1.0,)
			* 0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7,)).sin())
	}
}

pub struct ImageTexture {
	data:               Option<Vec<u8,>,>,
	width:              usize,
	height:             usize,
	bytes_per_scanline: usize,
}

impl ImageTexture {
	const BYTES_PER_PIXEL: usize = 3;

	pub fn new(name: &str,) -> Self {
		let (mut width, mut height,) = (0, 0,);
		let data = open(name,).map_or_else(
			|e| {
				eprintln!("{}", e.to_string());
				None
			},
			|image| {
				(width, height,) = image.dimensions();
				Some(image.into_rgb8().into_raw(),)
			},
		);
		let bytes_per_scanline = ImageTexture::BYTES_PER_PIXEL * width as usize;

		ImageTexture {
			data,
			width: width as usize,
			height: height as usize,
			bytes_per_scanline,
		}
	}
}

impl Texture for ImageTexture {
	fn value(&self, mut u: f64, mut v: f64, p: &Point3,) -> Color {
		match &self.data {
			None => Color::new(0.0, 1.0, 1.0,),
			Some(data,) => {
				u = clamp(u, 0.0, 1.0,);
				// todo fix vertical mirror the proper way, maybe it's the image crate maybe it's me
				v = 1.0 - clamp(v, 0.0, 1.0,);

				let mut i = (u * self.width as f64) as usize;
				let mut j = (v * self.height as f64) as usize;

				if i >= self.width {
					i = self.width - 1;
				}
				if j >= self.height {
					j = self.height - 1;
				}

				let color_scale = 1.0 / 255.0;
				let index = j * self.bytes_per_scanline + i * ImageTexture::BYTES_PER_PIXEL;
				let pixel = &data[index .. index + 3];

				Color::new(
					color_scale * pixel[0] as f64,
					color_scale * pixel[1] as f64,
					color_scale * pixel[2] as f64,
				)
			},
		}
	}
}
