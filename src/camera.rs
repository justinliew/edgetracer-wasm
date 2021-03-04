use crate::vec3::{Vec3,Point3};
use crate::ray::Ray;

pub struct Camera {
	origin: Point3,
	lower_left: Point3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new(ar: f64, vfov: f64) -> Self {

		let theta = vfov.to_radians();
		let h = f64::tan(theta / 2.0);
		let viewport_height = 2.0 * h;
		let viewport_width : f64 = ar * viewport_height;

		const FOCAL_LENGTH : f64 = 1.0;

		let origin = Point3::new(0.0,0.0,0.0);
		let horizontal = Point3::new(viewport_width, 0.0,0.0);
		let vertical = Point3::new(0.0, viewport_height, 0.0);
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