use crate::vec3::{Vec3,Point3};
use crate::ray::Ray;

pub struct Camera {
	origin: Point3,
	lower_left: Point3,
	horizontal: Vec3,
	vertical: Vec3,
}

impl Camera {
	pub fn new(ar: f64, vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3) -> Self {

		let theta = vfov.to_radians();
		let h = f64::tan(theta / 2.0);
		let viewport_height = 2.0 * h;
		let viewport_width : f64 = ar * viewport_height;

		let w = Vec3::unit_vector(lookfrom - lookat);
		let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
		let v = Vec3::cross(&w, &u);


		let origin = lookfrom;
		let horizontal = u * viewport_width;
		let vertical = v * viewport_height;
		Camera{
			origin: origin,
			lower_left: origin - horizontal/2.0 - vertical/2.0 - w,
			horizontal: horizontal,
			vertical: vertical,
		}
	}

	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		Ray::new(&self.origin, &(self.lower_left + self.horizontal * s + self.vertical * t - self.origin))
	}
}