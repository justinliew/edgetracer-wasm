use crate::vec3::{Point3,Vec3};
use crate::ray::{Ray};
use crate::hittable::{HitRecord,Hittable};

pub struct Sphere {
	centre: Point3,
	radius: f64,
}

impl Sphere {
	pub fn new(c: Point3, r: f64) -> Self {
		Sphere{
			centre: c,
			radius: r
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
		let oc = r.origin - self.centre;
		let a = r.dir.len_sq();
		let half_b = Vec3::dot(&oc, &r.dir);
		let c = oc.len_sq() - self.radius * self.radius;
		let disc = half_b * half_b - a*c;
		if disc < 0.0 {
			return None;
		}

		let root = (-half_b - disc.sqrt()) / a;
		if root < tmin || root > tmax {
			return None;
		}

		let p = r.at(root);
		let outward_normal = (p - self.centre) / self.radius;
		Some(HitRecord::new(r, root, &outward_normal))
	}
}