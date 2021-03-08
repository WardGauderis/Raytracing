use crate::{
	util::random_i32_in_range,
	vec3::{dot, unit_vector, Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
	ranvec: Box<[Vec3; POINT_COUNT],>,
	perm_x: Box<[usize; POINT_COUNT],>,
	perm_y: Box<[usize; POINT_COUNT],>,
	perm_z: Box<[usize; POINT_COUNT],>,
}

impl Perlin {
	pub fn new() -> Self {
		let mut perlin = Perlin {
			ranvec: Box::new([Vec3::default(); POINT_COUNT],),
			perm_x: Self::perlin_generate_perm(),
			perm_y: Self::perlin_generate_perm(),
			perm_z: Self::perlin_generate_perm(),
		};

		for vec in &mut *perlin.ranvec {
			*vec = unit_vector(&Vec3::random_in_range(-1.0, 1.0,),);
		}

		perlin
	}

	pub fn noise(&self, p: &Point3,) -> f64 {
		let mut u = p.x() - p.x().floor();
		let mut v = p.y() - p.y().floor();
		let mut w = p.z() - p.z().floor();
		let i = p.x().floor() as i32;
		let j = p.y().floor() as i32;
		let k = p.z().floor() as i32;
		let mut c = [[[Vec3::default(); 2]; 2]; 2];

		for di in 0 .. 2 {
			for dj in 0 .. 2 {
				for dk in 0 .. 2 {
					c[di][dj][dk] = self.ranvec[self.perm_x[((i + di as i32) & 255) as usize]
						^ self.perm_y[((j + dj as i32) & 255) as usize]
						^ self.perm_z[((k + dk as i32) & 255) as usize]]
				}
			}
		}

		Self::perlin_interp(&c, u, v, w,)
	}

	pub fn turb(&self, p: &Point3, depth: i32,) -> f64 {
		let mut accum = 0.0;
		let mut temp_p = p.clone();
		let mut weight = 1.0;

		for i in 0 .. depth {
			accum += weight * self.noise(&temp_p,);
			weight *= 0.5;
			temp_p *= 2.0;
		}

		accum.abs()
	}

	fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64,) -> f64 {
		let uu = u * u * (3.0 - 2.0 * u);
		let vv = v * v * (3.0 - 2.0 * v);
		let ww = w * w * (3.0 - 2.0 * w);
		let mut accum = 0.0;

		for i in 0 .. 2 {
			for j in 0 .. 2 {
				for k in 0 .. 2 {
					let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64,);
					accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
						* (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
						* (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
						* dot(&c[i][j][k], &weight_v,);
				}
			}
		}

		accum
	}

	fn perlin_generate_perm() -> Box<[usize; POINT_COUNT],> {
		let mut p = Box::new([0; POINT_COUNT],);

		for i in 0 .. POINT_COUNT {
			p[i] = i;
		}

		Self::permute(&mut p, POINT_COUNT,);

		p
	}

	fn permute(p: &mut Box<[usize; POINT_COUNT],>, n: usize,) {
		for i in (1 .. n).into_iter().rev() {
			let target = random_i32_in_range(0, i as i32,) as usize;
			p.swap(i, target,);
		}
	}
}
