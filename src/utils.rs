use crate::vec3::{Vec3};

pub fn rand_in_unit_sphere() -> Vec3 {
	loop {
		let p = Vec3::random_range(-1.0,1.0);
		if p.len_sq() < 1.0 {
			return p;
		}
	}
}

pub fn rand_unit_vector() -> Vec3 {
	Vec3::unit_vector(rand_in_unit_sphere())
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
	if x < min {
		min
	} else if x > max {
		max
	} else {
		x
	}
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
	*v -  *n * Vec3::dot(v,n) * 2.
}