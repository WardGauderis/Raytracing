use rand::prelude::*;
use std::f64::consts::PI;

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
	x.min(max).max(min)
}

#[inline]
pub fn random_f64() -> f64 {
	rand::thread_rng().gen()
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
	rand::thread_rng().gen_range(min..max)
}

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
	degrees * PI / 180.0
}

#[inline]
pub fn random_i32_in_range(min: i32, max: i32) -> i32 {
	random_f64_range(min as f64, (max + 1) as f64) as i32
}
