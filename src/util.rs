use rand::prelude::*;

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