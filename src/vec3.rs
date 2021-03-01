use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
	pub x : f64,
	pub y : f64,
	pub z : f64,
}

impl ops::Add for Vec3 {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::Sub for Vec3 {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::Mul<f64> for Vec3 {
	type Output = Vec3;
	fn mul(self, multiplier: f64) -> Self  {
		Self {
			x: self.x * multiplier,
			y: self.y * multiplier,
			z: self.z * multiplier,
		}
	}
}

impl ops::Div<f64> for Vec3 {
	type Output = Vec3;
	fn div(self, divisor: f64) -> Self  {
		Self {
			x: self.x / divisor,
			y: self.y / divisor,
			z: self.z / divisor,
		}
	}
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self{x: x, y: y, z: z}
	}

	fn len_sq(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	fn len(&self) -> f64 {
		self.len_sq().sqrt()
	}

	// TODO - reference to self? operator overload needs fixing
	pub fn unit_vector(self) -> Vec3 {
		self / self.len()
	}
}

pub type Colour = Vec3;
pub type Point3 = Vec3;