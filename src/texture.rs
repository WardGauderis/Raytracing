use crate::vec3::{Color, Point3, Vec3};
use std::rc::Rc;

pub trait Texture {
	fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

#[derive(Default)]
pub struct SolidColor {
	color_value: Color,
}

impl SolidColor {
	pub fn new(red: f64, green: f64, blue: f64) -> Self {
		SolidColor {
			color_value: Color::new(red, green, blue),
		}
	}
}

impl From<Color> for SolidColor {
	fn from(color: Color) -> Self {
		SolidColor { color_value: color }
	}
}

impl Texture for SolidColor {
	fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
		self.color_value
	}
}

pub struct CheckerTexture {
	even: Rc<dyn Texture>,
	odd: Rc<dyn Texture>,
}

impl CheckerTexture {
	pub fn new(even: Color, odd: Color) -> Self {
		CheckerTexture {
			even: Rc::new(SolidColor::from(even)),
			odd: Rc::new(SolidColor::from(odd)),
		}
	}
}

impl Texture for CheckerTexture {
	fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
		let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
		return if sines < 0.0 {
			self.odd.value(u, v, p)
		} else {
			self.even.value(u, v, p)
		};
	}
}
