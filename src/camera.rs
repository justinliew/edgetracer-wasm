use crate::vec3::{Vec3,Point3};
use crate::ray::Ray;

pub struct Camera {
	origin: Point3,
	lower_left: Point3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new(ar: f64) -> Self {

		const VIEWPORT_HEIGHT : f64 = 2.0;
		let viewport_width : f64 = ar * VIEWPORT_HEIGHT;
		const FOCAL_LENGTH : f64 = 1.0;

		let origin = Point3::new(0.0,0.0,0.0);
		let horizontal = Point3::new(viewport_width, 0.0,0.0);
		let vertical = Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
		Camera{
			origin: origin,
			lower_left: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,FOCAL_LENGTH),
			horizontal: horizontal,
			vertical: vertical,
		}
	}

	pub fn ray(&self, u: f64, v: f64) -> Ray {
		Ray::new(&self.origin, &(self.lower_left + self.horizontal * u + self.vertical * v - self.origin))
	}
}