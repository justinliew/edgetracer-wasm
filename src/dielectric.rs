use crate::vec3::{Colour,Vec3};
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::{refract};

pub struct Dielectric {
	ir: f64,
}

impl Dielectric {
	pub fn new(ir: f64) -> Self {
		Dielectric{
			ir: ir,
		}
	}

}
impl Material for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)> {
		let refraction_ratio = match rec.front {
			true => 1.0/self.ir,
			false => self.ir
		};
		let unit_direction = Vec3::unit_vector(r_in.dir);
		let refracted = refract(&unit_direction, &rec.normal, refraction_ratio);

		Some(
			(
				Ray::new(&rec.p, &refracted),
				Colour::new(1.0,1.0,1.0),
			)
		)
	}
}